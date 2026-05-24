self.send_to_host = (func, args, retAddr) => {
    const obj = { func, args, retAddr };
    self.postMessage(obj);
};
async function run(module, memory) {
    const exe = await import(module);
    await exe.default(/* module */ undefined, memory);
    exe.main();
}
self.onmessage = (e) => {
    const { module, memory } = e.data;
    run(module, memory).catch((e) => console.error(e));
};
export {};
//# sourceMappingURL=worker.js.map