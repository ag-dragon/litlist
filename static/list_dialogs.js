const createDialog = document.querySelector("#story_create_dialog");
const createShowButton = document.querySelector("#show_story_create");
const createCloseButton = document.querySelector("#close_story_create");

createShowButton.addEventListener("click", () => {
    createDialog.showModal();
});

createCloseButton.addEventListener("click", () => {
    createDialog.close();
});

const editDialogs = document.querySelectorAll(".story_edit_dialog");
const editShowButtons = document.querySelectorAll(".show_story_edit");
const editCloseButtons = document.querySelectorAll(".close_story_edit");

editShowButtons.forEach((editShowButton, index) => {
    editShowButton.addEventListener("click", () => {
        editDialogs[index].showModal();
    });
});

editCloseButtons.forEach((editCloseButton, index) => {
    editCloseButton.addEventListener("click", () => {
        editDialogs[index].close();
    });
});
