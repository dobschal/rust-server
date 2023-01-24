fetch("/users").then(async rawResponse => {
    const response = await rawResponse.json();
    console.log("Response: ", response);
});