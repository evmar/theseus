class MessageQueue {
    messages = [];
    waiter;
    poll() {
        return this.messages.shift();
    }
    wait() {
        const msg = this.poll();
        if (msg !== undefined) {
            return Promise.resolve(msg);
        }
        const { promise, resolve } = Promise.withResolvers();
        this.waiter = resolve;
        return promise;
    }
    enqueue = (e) => {
        e.preventDefault();
        if (this.waiter) {
            this.waiter(e);
            this.waiter = undefined;
        }
        else {
            this.messages.push(e);
        }
    };
    discard = (e) => {
        e.preventDefault();
    };
    listen(dom) {
        dom.onmousedown = this.enqueue;
        dom.onmouseup = this.enqueue;
        dom.onmousemove = this.enqueue;
        dom.oncontextmenu = this.discard;
    }
}
class Host {
    wasmMemory;
    consoleDom = document.createElement("pre");
    consoleOutput = new ArrayBuffer(0, { maxByteLength: 10 << 10 });
    window_;
    surfaces = new Map();
    nextSurface = 1;
    messageQueue = new MessageQueue();
    constructor(wasmMemory) {
        this.wasmMemory = wasmMemory;
        this.consoleDom.id = "console";
        document.body.appendChild(this.consoleDom);
    }
    onMessage(e) {
        const msg = e.data;
        const ret = this[msg.func](...msg.args);
        if (msg.retAddr) {
            if (ret instanceof Promise) {
                ret.then((ret) => this.finishSync(msg.retAddr, ret));
                return;
            }
            this.finishSync(msg.retAddr, ret);
        }
    }
    finishSync(retAddr, ret) {
        const arr = Array.isArray(ret) ? ret : [ret];
        if (!Number.isFinite(arr[0]) || arr[0] == 0) {
            // For synchronization to work, we must put a non-zero value in the first slot.
            // If this hits we messed up the sync/non-sync ness of some API.
            throw new Error();
        }
        const ints = new Int32Array(this.wasmMemory.buffer, retAddr, arr.length);
        ints.set(arr);
        Atomics.notify(ints, 0, 1);
    }
    console_write(ptr, len) {
        const inBuf = new Uint8Array(this.wasmMemory.buffer, ptr, len);
        const ofs = this.consoleOutput.byteLength;
        this.consoleOutput.resize(ofs + len);
        const outBuf = new Uint8Array(this.consoleOutput, ofs, len);
        outBuf.set(inBuf);
        this.consoleDom.innerText = new TextDecoder().decode(this.consoleOutput);
    }
    create_surface(width, height) {
        const surface = document.createElement("canvas");
        surface.width = width;
        surface.height = height;
        document.body.appendChild(surface);
        const id = this.nextSurface++;
        this.surfaces.set(id, surface);
        return id;
    }
    create_window(title, width, height) {
        this.window_ = document.createElement("canvas");
        this.window_.className = "window";
        this.window_.width = width;
        this.window_.height = height;
        document.body.appendChild(this.window_);
        this.messageQueue.listen(this.window_);
        return 1;
    }
    resize_window(id, width, height) {
        this.window_.width = width;
        this.window_.height = height;
    }
    render(window_id, surface_id) {
        const surface = this.surfaces.get(surface_id);
        this.window_.getContext("2d").drawImage(surface, 0, 0);
    }
    set_pixels(id, ptr, len) {
        const copy = new Uint8ClampedArray(this.wasmMemory.buffer, ptr, len).slice();
        const surface = this.surfaces.get(id);
        const imageData = new ImageData(copy, surface.width);
        surface.getContext("2d").putImageData(imageData, 0, 0);
        return 1;
    }
    serializeMessage(event) {
        // see wasm.rs:parse_message
        const typeToCode = {
            mousedown: 2,
            mouseup: 3,
            mousemove: 4,
        };
        const code = typeToCode[event.type];
        if (code === undefined)
            throw new Error();
        switch (event.type) {
            case "mousedown":
            case "mouseup":
            case "mousemove": {
                const e = event;
                return [typeToCode[e.type], e.offsetX, e.offsetY, 1 << e.button];
            }
            default:
                throw new Error();
        }
    }
    poll_message() {
        const event = this.messageQueue.poll();
        return event ? this.serializeMessage(event) : [-1];
    }
    async wait_message() {
        const event = await this.messageQueue.wait();
        return this.serializeMessage(event);
    }
}
async function main() {
    if (!window.SharedArrayBuffer) {
        document.body.innerText =
            "SharedArrayBuffer is not supported; possibly try reloading";
        return;
    }
    const memory = new WebAssembly.Memory({
        initial: 32, // in units of 64KB pages, 2mb
        maximum: 1024, // 64mb
        shared: true,
    });
    const host = new Host(memory);
    const worker = new Worker("./worker.js", { type: "module" });
    worker.onmessage = (e) => host.onMessage(e);
    const message = {
        module: "./exe/mine/mine.js",
        memory,
    };
    worker.postMessage(message);
}
main().catch((e) => console.error(e));
export {};
//# sourceMappingURL=host.js.map