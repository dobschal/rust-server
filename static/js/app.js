const listElement = document.querySelector("ul");
const formElement = document.querySelector("form");
const inputElement = document.querySelector("input");

loadUsers();

formElement.addEventListener("submit", async e => {
    e.preventDefault();
    const name = inputElement.value;
    await fetch("/users", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            name
        })
    });
    loadUsers();
    inputElement.value = "";
    inputElement.focus();
});

/**
 * @param {{ tag: string, text: string, parent: HTMLElement }} param0 
 * @returns {HTMLElement}
 */
function createElement({ tag = "div", text = "", parent }) {
    const element = document.createElement(tag);
    element.innerText = text;
    if (parent) {
        parent.appendChild(element);
    }
    return element;
}

function loadUsers() {
    fetch("/users").then(async rawResponse => {
        listElement.innerHTML = "";
        const users = await rawResponse.json();
        console.log("[app] Got users: ", users);
        users.forEach(user => {
            createElement({
                tag: "li",
                text: user.name,
                parent: listElement
            });
        });
    });
}