window.addEventListener('DOMContentLoaded', async (_event) => {
  if(!document.querySelector('video')) return;

  await loadState();

  // Configure mediaSession
  navigator.mediaSession.metadata = new MediaMetadata({
    title: data.title,
    artist: data.channel,
  });

  bindKeys();

  setInterval(storeState, 5000);
});

async function loadState() {
  try {
    let response = await fetch(`/api/playstates/${data.id}`);
    let playstate = await response.json();

    console.log('Loading state', playstate);

    let video = document.querySelector('video');
    video.currentTime = playstate.position;
  } catch {
    console.log('No playstate');
  }
}

async function storeState() {
  let video = document.querySelector('video');

  let playstate = {
    id: data.id,
    position: Math.round(video.currentTime),
  };

  console.log('Storing state', playstate);

  await fetch(`/api/playstates`, {
    method: 'PUT',
    body: JSON.stringify(playstate),
    headers: {
      'Content-Type': 'application/json',
    },
  });
}

function bindKeys() {
  let video = document.querySelector('video');
  document.addEventListener('keydown', ev => {
    let modifierPressed = ev.ctrlKey || ev.altKey || ev.metaKey || ev.shiftKey;
    if(modifierPressed) return;
    if((ev.key == ' ' || ev.key == 'k') && document.webkitFullscreenElement == null) {
      ev.preventDefault();

      video.paused ? video.play() : video.pause();
    } else if(ev.key == 'l') {
      ev.preventDefault();

      video.currentTime += 5;
    } else if(ev.key == 'j') {
      ev.preventDefault();

      video.currentTime -= 5;
    } else if(ev.key == '0') {
      ev.preventDefault();

      video.currentTime = 0;
    } else if(ev.key == 'f') {
      ev.preventDefault();

      if(document.webkitFullscreenElement == null) {
        video.webkitRequestFullscreen();
      } else {
        document.webkitExitFullscreen();
      }
    }
  });
}
