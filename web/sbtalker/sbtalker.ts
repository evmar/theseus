import type { SBTalker } from "../exe/sbaitso_sbtalker/sbaitso_sbtalker.js";
import * as exe from "../exe/sbaitso_sbtalker/sbaitso_sbtalker.js";

async function init_sbaitso(): Promise<SBTalker> {
  await exe.default();
  return exe.init();
}

function init_dom(sbtalker: SBTalker) {
  const form = document.getElementById('speak')! as HTMLFormElement;
  form.addEventListener('submit', (e) => {
    e.preventDefault();
    const data = new FormData(form);
    const text = data.get('text') as string;
    play_audio(sbtalker, text);
  });
}

function play_audio(sbtalker: SBTalker, text: string) {
  const buf = sbtalker.speak(text);
  const audioCtx = new AudioContext({ sampleRate: 8474 });
  const audioBuffer = audioCtx.createBuffer(1, buf.length, 8474);
  const channelData = audioBuffer.getChannelData(0);
  for (let i = 0; i < buf.length; i++) {
    channelData[i] = (buf[i]! - 128) / 128; // convert from unsigned 8-bit to float
  }
  const source = audioCtx.createBufferSource();
  source.buffer = audioBuffer;
  source.connect(audioCtx.destination);
  source.start();
}


async function main() {
  const sbtalker = await init_sbaitso();
  init_dom(sbtalker);
}

main().catch(err => console.error(err));
