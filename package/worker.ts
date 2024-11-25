self.onmessage = function(event) {
    const data = event.data;
    console.log('Worker received:', data);

    // Perform some computation or task
    const result = data * 2;

    // Send the result back to the main thread
    self.postMessage(result);
};