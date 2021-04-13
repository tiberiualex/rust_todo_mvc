function renderItems(items, processType, elementId, processFunction) {
  let placeholder = "<div>";
  let itemsMeta = [];

  for (i = 0; i < items.length; i++) {
    let title = items[i]["title"];
    let placeholderId = processType + "-" + title.replaceAll(" ", "-");
    placeholder += "<div>" + title + "<button " + 'id="' + placeholderId + '">' + processType + '</button>' + "</div>";

    itemsMeta.push({
      id: placeholderId,
      title: title
    });
  }

  placeholder += "</div>";
  document.getElementById(elementId).innerHtml = placeholder;

  for (i = 0; i < itemsMeta.length; i++) {
    document.getElementById(itemsMeta[i]["id"]).addEventListener("click", processFunction);
  }
}

function apiCall(url, method) {
  let xhr = new XMLHttpRequest();
  xhr.withCredentials = true;

  xhr.addEventListener('readystatechange', function() {
    if (this.readyState === this.DONE) {
      renderItems(JSON.parse(this.responseText)["pending_items"], "edit", "pendingItems", editItem);
      renderItems(JSON.parse(this.responseText)["done_items"], "delete", "doneItems", deleteItem);
    }
  });

  xhr.open(method, url);
  xhr.setRequestHeader('content-type', 'application/json');
  xhr.setRequestHeader('user-token', 'token');
  return xhr;
}
