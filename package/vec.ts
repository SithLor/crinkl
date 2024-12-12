class Vec<T> {
    private elements: T[];

    constructor() {
        this.elements = [];
    }

    // Add an element to the end of the vector
    push(element: T): void {
        this.elements.push(element);
    }

    // Remove and return the last element of the vector
    pop(): T | undefined {
        return this.elements.pop();
    }

    // Get the element at the specified index
    get(index: number): T | undefined {
        return this.elements[index];
    }

    // Get the length of the vector
    len(): number {
        return this.elements.length;
    }

    // Check if the vector is empty
    isEmpty(): boolean {
        return this.elements.length === 0;
    }

    // Clear the vector
    clear(): void {
        this.elements = [];
    }

    // Iterate over the elements of the vector
    *iter(): IterableIterator<T> {
        for (const element of this.elements) {
            yield element;
        }
    }
}

// Example usage
const vec = new Vec<number>();
vec.push(1);
vec.push(2);
vec.push(3);
//add lot of data
for (let i = 0; i < 10000; i++) {
    vec.push(i);
}


console.log(vec.get(0)); // Output: 1
console.log(vec.pop());  // Output: 3
console.log(vec.len());  // Output: 2
console.log(vec.isEmpty()); // Output: false

//add each element in one variable
let sum = 0;
for (const element of vec.iter()) {
    sum += element;
}
console.log(sum); // Output: 9999