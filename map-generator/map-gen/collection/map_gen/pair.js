class Pair {
    constructor(x,y) {
        this.x = x;
        this.y = y;
    }

    serialize() {
        return `${this.x}-${this.y}`
    }

    static from_string(s) {
        let [x,y] = s.split('-').map(x => parseInt(x))
        return new Pair(x,y)
    }
}