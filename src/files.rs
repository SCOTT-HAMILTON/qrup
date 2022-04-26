pub fn get_html_form() -> String {
    HTML_FORM
        .to_string()
        .replace("{:style}", CSS_FORM)
        .replace("{:script}", SCRIPT_FORM)
}

pub const HTML_SUCCESS: &str = r###"
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <title>QrUp File Upload</title>
        </head>
        <body>
            File successfully uploaded !
        </body>
        </html>
        "###;

pub const HTML_FORM: &str = r###"
<html lang="en">
	<head>
		<meta charset="UTF-8">

        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
		<title>QrUp File Upload</title>

        <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.2.1/css/bootstrap.min.css" integrity="sha384-GJzZqFGwb1QTTN6wy59ffF1BuGJpLSa9DkKMp0DgiMDm4iYMj70gZWKYbI706tWS" crossorigin="anonymous">

        <style charset="UTF-8">{:style}</style>
	</head>
	<body>

<div class="container">
    <div class="row">
      <div class="col">

        <div class="mb-3 mt-3">

          <h2 class="mb-3" style="font-weight: 300">Upload file</h2>

          <div class="form-group mb-3">
            <div class="custom-file">
              <input type="file" class="custom-file-input" name="file_input" id="file_input" oninput="input_filename();">
              <label id="file_input_label" class="custom-file-label" for="image">Select file</label>
            </div>
          </div>

          <button onclick="upload('');" id="upload_btn" class="btn btn-primary">Upload</button>

          <button class="btn btn-primary d-none" id="loading_btn" type="button" disabled>
            <span class="spinner-border spinner-border-sm" role="status" aria-hidden="true"></span>
            Uploading...
          </button>

          <button type="button" id="cancel_btn" class="btn btn-secondary d-none">Cancel upload</button>

        </div>

        <div id="progress_wrapper" class="d-none">
          <label id="progress_status"></label>
          <div class="progress mb-3">
            <div id="progress" class="progress-bar" role="progressbar" aria-valuenow="25" aria-valuemin="0" aria-valuemax="100"></div>
          </div>
        </div>

        <div id="alert_wrapper"></div>

      </div>
    </div>
  </div>

        <script charset="UTF-8">
        {:script}
        </script>
        <script src="https://code.jquery.com/jquery-3.3.1.slim.min.js"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.6/umd/popper.min.js"></script>
        <script src="https://stackpath.bootstrapcdn.com/bootstrap/4.2.1/js/bootstrap.min.js"></script>

	</body>
</html>
        "###;

pub const CSS_FORM: &str = r###"
    "###;

pub const SCRIPT_FORM: &str = r###"
    // Get a reference to the progress bar, wrapper & status
    var progress = document.getElementById("progress");
    var progress_wrapper = document.getElementById("progress_wrapper");
    var progress_status = document.getElementById("progress_status");

    // Get a reference to the 3 buttons
    var upload_btn = document.getElementById("upload_btn");
    var loading_btn = document.getElementById("loading_btn");
    var cancel_btn = document.getElementById("cancel_btn");

    // Get a reference to the alert wrapper
    var alert_wrapper = document.getElementById("alert_wrapper");

    // Get a reference to the file input element & input label 
    var input = document.getElementById("file_input");
    var file_input_label = document.getElementById("file_input_label");

    // Function to show alerts
    function show_alert(message, alert) {

      alert_wrapper.innerHTML = `
        <div id="alert" class="alert alert-${alert} alert-dismissible fade show" role="alert">
          <span>${message}</span>
          <button type="button" class="close" data-dismiss="alert" aria-label="Close">
            <span aria-hidden="true">&times;</span>
          </button>
        </div>
      `

    }

    // Function to upload file
    function upload(url) {

      // Reject if the file input is empty & throw alert
      if (!input.value) {

        show_alert("No file selected", "warning")

        return;

      }

      // Create a new FormData instance
      var data = new FormData();

      // Create a XMLHTTPRequest instance
      var request = new XMLHttpRequest();

      // Set the response type
      request.responseType = "json";

      // Clear any existing alerts
      alert_wrapper.innerHTML = "";

      // Disable the input during upload
      input.disabled = true;

      // Hide the upload button
      upload_btn.classList.add("d-none");

      // Show the loading button
      loading_btn.classList.remove("d-none");

      // Show the cancel button
      cancel_btn.classList.remove("d-none");

      // Show the progress bar
      progress_wrapper.classList.remove("d-none");

      // Get a reference to the file
      var file = input.files[0];

      // Get a reference to the filename
      var filename = file.name;

      // Get a reference to the filesize & set a cookie
      var filesize = file.size;
      document.cookie = `filesize=${filesize}`;

      // Append the file to the FormData instance
      data.append("file", file);

      // request progress handler
      request.upload.addEventListener("progress", function (e) {

        // Get the loaded amount and total filesize (bytes)
        var loaded = e.loaded;
        var total = e.total

        // Calculate percent uploaded
        var percent_complete = (loaded / total) * 100;

        // Update the progress text and progress bar
        progress.setAttribute("style", `width: ${Math.floor(percent_complete)}%`);
        progress_status.innerText = `${Math.floor(percent_complete)}% uploaded`;

      })

      // request load handler (transfer complete)
      request.addEventListener("load", function (e) {

        if (request.status == 200) {

          show_alert(`File successfully uploaded !`, "success");

        }
        else {

          show_alert(`Error uploading file`, "danger");

        }

        reset();

      });

      // request error handler
      request.addEventListener("error", function (e) {

        reset();

        show_alert(`Error uploading file`, "warning");

      });

      // request abort handler
      request.addEventListener("abort", function (e) {

        reset();

        show_alert(`Upload cancelled`, "primary");

      });

      // Open and send the request

      fetch(window.location.href+'start');
      request.open("post", url);
      request.send(data);

      cancel_btn.addEventListener("click", function () {
        request.abort();
      })

    }

    // Function to update the input placeholder
    function input_filename() {

      file_input_label.innerText = input.files[0].name;

    }

    // Function to reset the page
    function reset() {

      // Clear the input
      input.value = null;

      // Hide the cancel button
      cancel_btn.classList.add("d-none");

      // Reset the input element
      input.disabled = false;

      // Show the upload button
      upload_btn.classList.remove("d-none");

      // Hide the loading button
      loading_btn.classList.add("d-none");

      // Hide the progress bar
      progress_wrapper.classList.add("d-none");

      // Reset the progress bar state
      progress.setAttribute("style", `width: 0%`);

      // Reset the input placeholder
      file_input_label.innerText = "Select file";

    }
        "###;
