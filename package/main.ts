const worker = new Worker('worker.ts');

worker.onmessage = function(event) {
    const result = event.data;
    console.log('Main thread received:', result);
};

function add(i,e){
    let fuckthis = i+e

    return fuckthis
}

const data = 42;
console.log('Main thread sending:', data);
worker.postMessage(data);