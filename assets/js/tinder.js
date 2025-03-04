// General functions for Tinder
function openPromoteDialog() {
    document.getElementById("promote-dialog").showModal();
}
function closePromoteDialog() {
    document.getElementById("promote-dialog").close();
}

// Keyboard shortcuts for desktop
window.addEventListener("keydown", (event) => {
    if (event.key === "ArrowLeft") {
        document.getElementById("promote-dialog").showModal();
    } else if (event.key === "ArrowRight") {
        document.querySelector("button#tinder-demote-submit-button").click();
    }
});

// Drag and drop for mobile
function initDragDrop() {
    function dropped(event) {
        drag_drop.classList.add('visually-hidden');
        document.getElementById('tinder-card').classList.remove('visually-hidden');
    }

    const drag_drop = document.getElementById('drag-drop');
    document.getElementById('tinder-card').addEventListener('dragstart', (event) => {
        setTimeout(() => {
            event.target.classList.add('visually-hidden')
            drag_drop.classList.remove('visually-hidden');
        }, 0);
    });
    for (let child of drag_drop.children) {
        child.addEventListener('dragenter', (event) => {
            event.preventDefault();
            child.classList.add('drag-hover');
        });
        child.addEventListener('dragleave', (event) => {
            child.classList.remove('drag-hover');
        });
        child.addEventListener('dragover', (event) => {
            event.preventDefault();
        });
    }

    document.getElementById('drag-promote').addEventListener('drop', (event) => {
        dropped(event);
        document.getElementById("promote-dialog").showModal();
    });
    document.getElementById('drag-cancel').addEventListener('drop', dropped);
    document.getElementById('drag-demote').addEventListener('drop', (event) => {
        dropped(event);
        document.querySelector("button#tinder-demote-submit-button").click();
    });
}

document.addEventListener('htmx:afterRequest', (event) => {
    // https://htmx.org/events/#htmx:afterRequest
    initDragDrop();
});

window.addEventListener('DOMContentLoaded', (event) => initDragDrop());
