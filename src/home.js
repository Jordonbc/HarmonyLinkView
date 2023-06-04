export const { invoke } = window.__TAURI__.tauri;
export const listen = window.__TAURI__.event.listen;

// Mapping from keys to display names
const DISPLAY_NAMES = {
  "os_info": "Operating System Info",
  "battery_info": "Battery Info",
  "dock_info": "Dock Info",
  // Add more key-display name pairs as needed
};

function update_ui_elements(info) {
  const container = document.querySelector('.container');

  // Clear the container first
  container.innerHTML = '';

  // Check if the payload is an error message
  if (info.payload && info.payload.Err) {
    // Server sent an error message, display it
    let errorMessage = document.createElement('div');
    errorMessage.className = 'error-message';

    if (info.payload.Err.includes('actively refused it')) {
      errorMessage.textContent = 'Error: The server refused the connection. Please make sure the server is running and try again.';
    } else {
      errorMessage.textContent = 'Error: ' + info.payload.Err;
    }
    
    container.appendChild(errorMessage);
  } else if (!info.payload || !info.payload.Ok) {
    // Server is offline or data could not be loaded, display error message
    let errorMessage = document.createElement('div');
    errorMessage.className = 'error-message';

    errorMessage.textContent = 'Sorry, the data could not be loaded. Please check your connection and try again.';
    container.appendChild(errorMessage);
  } else {
    const data = info.payload.Ok;
    console.log(data);

    // Loop through all sections
    Object.keys(data).forEach((section) => {
      // Create a new div element for the section
      let sectionDiv = document.createElement('div');
      sectionDiv.className = 'info-block';
      sectionDiv.id = section;

      // Create a new h2 element for the section header
      let header = document.createElement('h2');
      header.textContent = DISPLAY_NAMES[section] || section; // Use display name if available, else use key

      // Create a new ul element for the section content
      let ul = document.createElement('ul');

      // Loop through all data points in the section
      Object.keys(data[section]).forEach((key) => {
        // Create a new li element for each data point
        let li = document.createElement('li');
        li.id = key;

        if (typeof data[section][key] === 'boolean') {
          // Display tick for true and cross for false
          const tickCross = document.createElement('span');
          tickCross.className = 'tick-cross';
          tickCross.innerHTML = data[section][key] ? '&#10004;' : '&#10006;';
          tickCross.style.color = data[section][key] ? 'green' : 'red';

          li.innerHTML = `${key.replace(/_/g, ' ')}: `;
          li.appendChild(tickCross);
        } else if (typeof data[section][key] === 'number') {
          li.textContent = `${key.replace(/_/g, ' ')}: ${data[section][key]}`;
        } else {
          li.textContent = data[section][key]
            ? `${key.replace(/_/g, ' ')}: ${data[section][key]}`
            : `No data available for ${key.replace(/_/g, ' ')}`;
        }

        // Add li to ul
        ul.appendChild(li);
      });

      // Add header and ul to sectionDiv
      sectionDiv.appendChild(header);
      sectionDiv.appendChild(ul);

      // Add sectionDiv to container
      container.appendChild(sectionDiv);
    });
  }
}


window.addEventListener("DOMContentLoaded", async () => {
  console.log("Running home.js");

  await listen("update_frontend", update_ui_elements);
});