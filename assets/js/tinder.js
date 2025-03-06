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

function initDesktopDragDrop() {
    // Start of Drag and Drop (drag)
    document.getElementById('tinder-card').addEventListener('dragstart', (event) => {
        setTimeout(() => {
            event.target.classList.add('visually-hidden')
            document.getElementById('drag-drop').classList.remove('visually-hidden');
        }, 0);
    });

    // While drag and drop
    for (let child of document.getElementById('drag-drop').children) {
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

    // Finish Drag and Drop (drop)
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

function initMobileDragDrop() {
    // Used elements
    const drag_drop = document.getElementById('drag-drop');
    const tinder_card = document.getElementById('tinder-card');
    // Drop targets
    const drag_promote = document.getElementById('drag-promote');
    const drag_cancel = document.getElementById('drag-cancel');
    const drag_demote = document.getElementById('drag-demote');

    let startX = 0, startY = 0;
    let diffX = 0;

    tinder_card.addEventListener('touchstart', (event) => {
        const touch = event.touches[0];
        startX = touch.clientX;
        startY = touch.clientY;
        drag_drop.classList.remove('visually-hidden');
    });

    tinder_card.addEventListener('touchmove', (event) => {
        event.preventDefault();
        const touch = event.touches[0];
        diffX = touch.clientX - startX;
        let diffY = touch.clientY - startY;

        tinder_card.style.transform = `translate(${diffX}px, ${diffY}px)`;

        drag_demote.classList.remove('drag-hover');
        drag_promote.classList.remove('drag-hover');
        drag_cancel.classList.remove('drag-hover');
        if (diffX > 50) {
            drag_demote.classList.add('drag-hover');
        } else if (diffX < -50) {
            drag_promote.classList.add('drag-hover');
        } else {
            drag_cancel.classList.add('drag-hover');
        }
    });

    tinder_card.addEventListener('touchend', (event) => {
        tinder_card.style.transform = '';

        dropped(event);

        if (diffX > 50) {
            document.querySelector("button#tinder-demote-submit-button").click();
        } else if (diffX < -50) {
            document.getElementById("promote-dialog").showModal();
        }
    });
}

function dropped(event) {
    document.getElementById('drag-drop').classList.add('visually-hidden');
    document.getElementById('tinder-card').classList.remove('visually-hidden');
}

function initDragDrop() {
    initDesktopDragDrop();
    initMobileDragDrop();
}

document.addEventListener('htmx:afterRequest', (event) => {
    // https://htmx.org/events/#htmx:afterRequest
    initDragDrop();
});

window.addEventListener('DOMContentLoaded', (event) => initDragDrop());

