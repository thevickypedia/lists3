pub fn get_content() -> String {
    r###"<!DOCTYPE html>
<!--suppress JSUnresolvedLibraryURL, JSUnresolvedReference -->
<html lang="en">
<head>
    <title>Rustic Bucket Listing</title>
    <meta property="og:type" content="FileBrowser">
    <meta content="This is a filebrowser for S3 buckets" name="description">
    <meta name="keywords" content="AWS, S3, Rust, FileBrowser, HTML, CSS, JavaScript, Jinja2">
    <meta name="author" content="Vignesh Rao">
    <meta content="width=device-width, initial-scale=1" name="viewport">
    <link name="favicon" property="og:image" rel="icon" href="https://thevickypedia.github.io/open-source/images/logo/rust.ico">
    <link name="apple-touch-icon" property="og:image" rel="apple-touch-icon" href="https://thevickypedia.github.io/open-source/images/logo/rust.png">
    <meta http-equiv="content-type" content="no-cache, no-store, must-revalidate">
    <meta http-equiv="refresh" content="no-cache">
    <style>
        .corner {
            position: absolute;
            top: 3%;
            right: 1px;
            bottom: 97%;
            display: inline-flex;
        }

        .corner p {
            font-size: 14px;
            font-family: 'Courier New', monospace !important;
            margin-right: 15px; /* Spacing between the joining <p> tags */
        }

        small {
            margin-left: 10px;
        }
    </style>
    <script>
        function getSpace(s, l) {
            let ret = "";
            while (s.length + ret.length < l) {
                ret = ret + " ";
            }
            return ret;
        }

        location.querystring = (function () {
            // The return is a collection of key/value pairs
            let result = {};

            // Gets the query string with a preceeding '?'
            let querystring = location.search;

            // document.location.search is empty if a query string is absent
            if (!querystring)
                return result;

            // substring(1) to remove the '?'
            let pairs = querystring.substring(1).split("&");
            let splitPair;

            // Load the key/values of the return collection
            for (let i = 0; i < pairs.length; i++) {
                splitPair = pairs[i].split("=");
                result[splitPair[0]] = splitPair[1];
            }

            return result;
        })();

        function createRequestObject() {
            let request_o; // Declare the variable to hold the object.
            // noinspection JSDeprecatedSymbols
            let browser = navigator.appName; // Find the browser name
            if (browser === "Microsoft Internet Explorer") {
                /* Create the object using MSIE's method */
                request_o = new ActiveXObject("Microsoft.XMLHTTP");
            } else {
                /* Create the object using other browser's method */
                request_o = new XMLHttpRequest();
            }
            return request_o; // return the object
        }

        /* You can get more specific with version information by using
          parseInt(navigator.appVersion)
          Which will extract an integer value containing the version
          of the browser being used. */
        /* The variable http will hold our new XMLHttpRequest object. */
        let http = createRequestObject();

        function getList(bucketName, regionName, folderNames, ignoreObjects) {
            let pretext = document.getElementById('pretext');
            pretext.innerHTML = "Amazon S3 Bucket list v2";

            let origin = `http://${bucketName}.s3-${regionName}.amazonaws.com`
            let responseType = "application/xml"
            let proxy = "{{ proxy_server }}"
            let endpoint = `${proxy}?origin=${origin}&output=${responseType}`
            http.open('get', endpoint, true)
            http.onreadystatechange = function () {
                handleList(folderNames, ignoreObjects)
            };
            http.send(null);
        }

        function filterByPrefix(files, folderNames, ignoreObjects) {
            let filteredFiles = [];
            let totalSize = 0;
            let filteredSize = 0;
            for (let i = 0; i < files.length; i++) {
                let fileName = files[i].getElementsByTagName('Key')[0].firstChild.data;
                let fileSize = files[i].getElementsByTagName('Size')[0].firstChild.data;
                totalSize += parseInt(fileSize);
                let ignoreFlag = false;
                for (let j = 0; j < ignoreObjects.length; j++) {
                    if (fileName === ignoreObjects[j]) {
                        ignoreFlag = true;
                        break;
                    }
                }
                if (!ignoreFlag) {
                    if (folderNames.length > 0) {
                        for (let k = 0; k < folderNames.length; k++) {
                            if (fileName.startsWith(folderNames[k])) {
                                filteredFiles.push(files[i]);
                                filteredSize += parseInt(fileSize);
                            }
                        }
                    } else {
                        filteredFiles.push(files[i]);
                    }
                }
            }
            let totalSizeField = document.getElementById('bucketSize');
            let totalCountField = document.getElementById('bucketCount');
            let filterCountField = document.getElementById('filterCount');
            let filteredSizeField = document.getElementById('filteredSize');

            let totalSizeConverted = sizeConverter(totalSize);

            totalSizeField.innerHTML = `Bucket Size: ${totalSizeConverted}`;
            totalCountField.innerHTML = `Total Objects: ${files.length}`;

            if (filteredFiles.length !== files.length) {
                filterCountField.innerHTML = `Filtered Objects: ${filteredFiles.length}`;
            }
            let filteredSizeConverted = sizeConverter(filteredSize);
            if (filteredSize !== 0 && filteredSize !== totalSize && filteredSizeConverted !== totalSizeConverted) {
                filteredSizeField.innerHTML = `Filtered Size: ${filteredSizeConverted}`;
            }
            if (filteredFiles.length === 0) {
                alert(`No objects were retrieved for the filters\n\nIN: ${JSON.stringify(folderNames)}\nNOT IN: ${JSON.stringify(ignoreObjects)}`);
            }
            return filteredFiles;
        }

        // Mask the full path in 'Object Name' column to improve readability
        // This functionality is available only for objects filtered with `--filter` flag
        function maskPath(objectName, folderNames) {
            let stripped = objectName;
            for (let folder of folderNames) {
                if (objectName.startsWith(folder)) {
                    stripped = objectName.slice(folder.length);
                    break;
                }
            }
            if (stripped.startsWith('/')) {
                stripped = stripped.slice(1);
            }
            return stripped;
        }

        function handleList(folderNames, ignoreObjects) {
            if (http.readyState === 4) { // Finished loading the response
                let response = http.responseXML;
                let unfiltered = response.getElementsByTagName('Contents');
                let filteredFiles = filterByPrefix(unfiltered, folderNames, ignoreObjects);
                let fileList = [];
                for (let i = 0; i < filteredFiles.length; i++) {
                    let fileData = [];
                    fileList[i] = fileData;
                    let size = filteredFiles[i].getElementsByTagName('Size')[0].firstChild.data;
                    let name = filteredFiles[i].getElementsByTagName('Key')[0].firstChild.data;
                    let storage = filteredFiles[i].getElementsByTagName('StorageClass')[0].firstChild.data;
                    let lastmod = filteredFiles[i].getElementsByTagName('LastModified')[0].firstChild.data;
                    fileData[0] = name;
                    fileData[1] = size;
                    fileData[2] = lastmod;
                    fileData[3] = storage;
                }
                fileList.sort(getSort());

                let tableBody = document.getElementById('bucket_body');
                tableBody.innerHTML = ''; // Clear existing table content

                for (let i = 0; i < fileList.length; i++) {
                    let fileData = fileList[i];
                    let name = fileData[0];
                    let size = fileData[1];
                    let lastmod = fileData[2];
                    let storage = fileData[3];
                    let row = document.createElement('tr');
                    let sizeCell = document.createElement('td');
                    sizeCell.innerHTML = sizeConverter(size);
                    let lastmodCell = document.createElement('td');
                    lastmodCell.innerHTML = lastmod;
                    let storageCell = document.createElement('td');
                    storageCell.innerHTML = storage;
                    let nameCell = document.createElement('td');
                    nameCell.innerHTML = "<a href=\"" + name + "\">" + maskPath(name, folderNames) + "</a>";
                    row.appendChild(sizeCell);
                    row.appendChild(lastmodCell);
                    row.appendChild(storageCell);
                    row.appendChild(nameCell);
                    tableBody.appendChild(row);
                }
            }
        }

        function getQueryVariable(variable) {
            let query = window.location.search.substring(1);
            let vars = query.split("&");
            for (let i = 0; i < vars.length; i++) {
                let pair = vars[i].split("=");
                if (pair[0] === variable) {
                    return pair[1];
                }
            }
            return null;
        }

        function sortSize(a, b) {
            if (parseInt(a[1]) > parseInt(b[1])) return 1;
            if (parseInt(a[1]) < parseInt(b[1])) return -1;
            return 0;
        }

        function sortSizeDesc(a, b) {
            return (-sortSize(a, b));
        }

        function sortLastmod(a, b) {
            if (a[2] > b[2]) return 1;
            if (a[2] < b[2]) return -1;
            return 0;
        }

        function sortLastmodDesc(a, b) {
            return (-sortLastmod(a, b));
        }

        function sortStorage(a, b) {
            if (a[2] > b[2]) return 1;
            if (a[2] < b[2]) return -1;
            return 0;
        }

        function sortStorageDesc(a, b) {
            return (-sortStorage(a, b));
        }

        function sortName(a, b) {
            if (a[0] > b[0]) return 1;
            if (a[0] < b[0]) return -1;
            return 0;
        }

        function sortNameDesc(a, b) {
            return -sortName(a, b);
        }

        function getSort() {
            let s = getQueryVariable("sort");
            let d = getQueryVariable("sortdir");
            if (s === 'size') {
                return d === 'desc' ? sortSizeDesc : sortSize
            }
            if (s === 'name') {
                return d === 'desc' ? sortNameDesc : sortName
            }
            if (s === 'lastmod') {
                return d === 'desc' ? sortLastmodDesc : sortLastmod
            }
            if (s === 'storage') {
                return d === 'desc' ? sortStorageDesc : sortStorage
            }
            return sortName;
        }

        function sizeConverter(byteSize) {
            let sizeName = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
            let index = Math.floor(Math.log(byteSize) / Math.log(1024));
            return (byteSize / Math.pow(1024, index)).toFixed(2) + " " + sizeName[index];
        }

        function getNextSortDir(sortCol) {
            if (sortCol === getQueryVariable("sort"))
                return getQueryVariable("sortdir") === 'desc' ? 'asc' : 'desc';
            return 'asc'
        }

        function getLinkSize() {
            window.location.href = "?sort=size&sortdir=" + getNextSortDir('size');
        }

        function getLinkName() {
            window.location.href = "?sort=name&sortdir=" + getNextSortDir('name');
        }

        function getLinkLastmod() {
            window.location.href = "?sort=lastmod&sortdir=" + getNextSortDir('lastmod');
        }

        function getLinkStorage() {
            window.location.href = "?sort=storage&sortdir=" + getNextSortDir('storage');
        }

    </script>
    <style>
        table {
            border-collapse: collapse;
            width: 100%;
            color: #333;
            font-family: Arial, sans-serif;
            font-size: 14px;
            text-align: left;
            border-radius: 10px;
            overflow: hidden;
            box-shadow: 0 0 20px rgba(0, 0, 0, 0.1);
            margin: 50px auto;
            /* margin: auto;
            margin-top: 50px;
            margin-bottom: 50px; */
        }

        table th {
            background-color: #0e42bd;
            color: #fff;
            font-weight: bold;
            padding: 10px;
            text-transform: uppercase;
            letter-spacing: 1px;
            border-top: 1px solid #fff;
            border-bottom: 1px solid #ccc;
        }

        table tr:nth-child(even) td {
            background-color: #f2f2f2;
        }

        table tr:hover td {
            background-color: #ffedcc;
        }

        table td {
            background-color: #fff;
            padding: 10px;
            border-bottom: 1px solid #ccc;
            font-weight: bold;
        }
    </style>
</head>
<body onLoad='getList(
    bucketName="{{ bucket_name }}",
    regionName="{{ region_name }}",
    folderNames={{ folder_names }},
    ignoreObjects={{ ignore_objects }}
);'>
<pre><small id="pretext"></small></pre>
<div class="corner">
    <p id="bucketSize"></p>
    <p id="bucketCount"></p>
    <p id="filteredSize"></p>
    <p id="filterCount"></p>
</div>
<br>
<table id="bucket_table">
    <thead>
    <tr>
        <th><a style="color:#FFF" href="javascript:getLinkSize()">Size</a></th>
        <th><a style="color:#FFF" href="javascript:getLinkLastmod()">Last Modified</a></th>
        <th><a style="color:#FFF" href="javascript:getLinkStorage()">Storage Class</a></th>
        <th><a style="color:#FFF" href="javascript:getLinkName()">Name</a></th>
        <!--<th><a href="?sort=name&sortdir=asc">Name</a></th>-->
    </tr>
    </thead>
    <tbody id="bucket_body">
    <!-- Table body content will be populated by JavaScript -->
    </tbody>
</table>
</body>
</html>
"###.to_string()
}
