/// Loads the HTML content for listing S3 contents with bootstrap template.
///
/// # Returns
///
/// Returns the HTML content as a `String`
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
    <!-- CSS and JS for Bootstrap -->
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/4.5.2/css/bootstrap.min.css">
    <link rel="stylesheet" href="https://cdn.datatables.net/1.10.25/css/dataTables.bootstrap4.min.css">
    <script src="https://code.jquery.com/jquery-3.4.1.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/twitter-bootstrap/4.5.2/js/bootstrap.min.js"></script>
    <script src="https://cdn.datatables.net/1.10.25/js/jquery.dataTables.min.js"></script>
    <script src="https://cdn.datatables.net/1.10.25/js/dataTables.bootstrap4.min.js"></script>
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

        function getList(bucketName, regionName, folderNames, ignoreObjects, proxyServer) {
            let origin = `http://${bucketName}.s3-${regionName}.amazonaws.com`
            let responseType = "application/xml"
            let endpoint = `${proxyServer}?origin=${origin}&output=${responseType}`
            http.open('get', endpoint, false)
            http.send();
            if (http.status === 200) {
                return handleList(http.responseXML, folderNames, ignoreObjects);
            } else {
                throw new Error('Request failed'); // Throw an error if the request fails
            }
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
                            if (fileName != folderNames[k] && fileName.startsWith(folderNames[k])) {
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

        function sizeConverter(byteSize) {
            let sizeName = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
            let index = Math.floor(Math.log(byteSize) / Math.log(1024));
            return (byteSize / Math.pow(1024, index)).toFixed(2) + " " + sizeName[index];
        }

        function sizeToBytes(size, suffix) {
            switch (suffix) {
                case 'kb':
                    size *= 1024;
                    break;
                case 'mb':
                    size *= 1024 * 1024;
                    break;
                case 'gb':
                    size *= 1024 * 1024 * 1024;
                    break;
            }
            return size;
        }

        function handleList(xmlResponse, folderNames, ignoreObjects) {
            let unfiltered = xmlResponse.getElementsByTagName('Contents');
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
            return {
                columns: [
                    { label: 'Object Name', field: 'name' },
                    { label: 'Size', field: 'size' },
                    'Last Modified',
                    'Storage Class',
                ],
                rows: fileList,
            };
        }
    </script>
    <script>
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

        function renderTable(bucketName, regionName, folderNames, ignoreObjects, proxyServer) {
            let pretext = document.getElementById('pretext');
            pretext.innerHTML = "<a href='https://crates.io/crates/lists3'>Rustic Bucket Listing - v{{ cargo_version }}</a>";

            const data = getList(bucketName, regionName, folderNames, ignoreObjects, proxyServer);

            // Populate table headers
            const tableHeaders = jQuery('#table-headers');
            data.columns.forEach(column => {
                const th = jQuery('<th>').text(column.label || column);
                tableHeaders.append(th);
            });

            // Populate table rows
            const tableBody = jQuery('#table-body');
            data.rows.forEach(rowData => {
                const tr = jQuery('<tr>');
                rowData.forEach(function (cellData, i) {
                    const td = jQuery('<td>');
                    if (i === 0) {
                        td.html(`<a href="${cellData}">${maskPath(cellData, folderNames)}</a>`);
                    } else if (i === 1) {
                        td.text(sizeConverter(cellData));
                    } else {
                        td.text(cellData);
                    }
                    tr.append(td);
                });
                tableBody.append(tr);
            });

            // Initialize Bootstrap table
            const datatable = jQuery('#datatable').DataTable({
                "scrollY": true, // Enable vertical scrolling
                columnDefs: [
                    { type: 'size', targets: 1 } // Apply custom sorting for Size column
                ],
            });

            // Define custom sorting function for Size column
            jQuery.fn.dataTable.ext.type.order['size-pre'] = function (data) {
                if (typeof data !== 'string') {
                    return data;
                }
                let matches = data.match(/[\d.]+/);
                if (!matches) {
                    return 0;
                }
                let suffix = data.slice(matches[0].length).trim().toLowerCase();
                let size = parseFloat(matches[0]);
                return sizeToBytes(size, suffix);
            };

            // Define function for advanced search
            jQuery('#advanced-search-button').on('click', function () {
                let value = jQuery('#advanced-search-input').val().trim();
                let [phrase, columns] = value.split(' in:');
                phrase = phrase.trim().toLowerCase();
                columns = columns ? columns.split(',').map(str => str.trim()) : [];

                datatable.search('').columns().search('').draw(); // Clear previous search

                // Apply new search
                if (phrase) {
                    datatable.search(phrase);
                }
                columns.forEach(col => {
                    datatable.column(col).search(phrase);
                });
                datatable.draw();
            });
        }
    </script>
</head>
<body onLoad='renderTable(
    bucketName="{{ bucket_name }}",
    regionName="{{ region_name }}",
    folderNames={{ folder_names }},
    ignoreObjects={{ ignore_objects }},
    proxyServer="{{ proxy_server }}"
);'>
<pre><small id="pretext"></small></pre>
<div class="corner">
    <p id="bucketSize"></p>
    <p id="bucketCount"></p>
    <p id="filteredSize"></p>
    <p id="filterCount"></p>
</div>
<br>
<div class="container mt-5">
    <div class="input-group mb-4">
        <!--suppress HtmlFormInputWithoutLabel -->
        <input type="text" class="form-control" id="advanced-search-input"
               placeholder="Search phrase in:column1,column2">
        <div class="input-group-append">
            <button class="btn btn-primary" id="advanced-search-button" type="button">
                <i class="fa fa-search"></i>
            </button>
        </div>
    </div>
    <div class="table-responsive">
        <table id="datatable" class="table table-striped">
            <thead>
            <tr id="table-headers">
                <!-- Headers will be added dynamically -->
            </tr>
            </thead>
            <tbody id="table-body">
            <!-- Rows will be added dynamically -->
            </tbody>
        </table>
    </div>
</div>
<br>
</body>
</html>
"###.to_string()
}
