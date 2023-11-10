const dialog = document.querySelector("#story_create_dialog");
const showButton = document.querySelector("#show_story_create");
const closeButton = document.querySelector("#close_story_create");

showButton.addEventListener("click", () => {
    dialog.showModal();
});

closeButton.addEventListener("click", () => {
    dialog.close();
});
