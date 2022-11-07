const R = 50;
const C = 50;
let cellSizeW 
let cellSizeH 
const PADDING = 15;
let winH;
let winW;

let currBox = [0,0];

function setup() {
	createCanvas(windowWidth, windowHeight);
	// put setup c
	winH = windowHeight - 2 * PADDING; 
	winW = windowWidth - 2 * PADDING;
	cellSizeH = winH / R * 1.0;
	cellSizeW = winW / C * 1.0;
}

const limit = (x, up,low) => Math.max(Math.min(x,up),low)

const toIndx = (x, y) =>  {
	let indx = limit(Math.floor((x-PADDING) / cellSizeW),C-1,0); 	
	let indy = limit(Math.floor((y-PADDING) / cellSizeH),R-1,0); 	
	return new Pair(indx,indy)
}

let selc = new Set();

const coordToBox = (a) => {
	return [PADDING + a.x * cellSizeW + cellSizeW / 2,PADDING + a.y * cellSizeH + cellSizeH / 2,cellSizeW / 2,cellSizeH/ 2]
}

function draw() {
	// put drawing code here
	background(51);

	for(let i = 0; i <= R; i++) {
		stroke(255);
		line(PADDING + i * cellSizeW,PADDING,PADDING + i * cellSizeW,winH + PADDING)
	}
	for(let i = 0; i <= C; i++) {
		stroke(255);
		line(PADDING,PADDING + (i/1.0) * cellSizeH,winW + PADDING,PADDING + i * cellSizeH)
	}

	fill(240,240,240,50);
	rectMode(CENTER)
	let co = toIndx(mouseX,mouseY)
	let [x,y,w,h] = coordToBox(co)
	rect(x,y,w,h)
	for(let arr of selc.keys()) {
		fill(23,240,240,100);
		rectMode(CENTER)
		let [x,y,w,h] = coordToBox(Pair.from_string(arr))
		rect(x,y,w,h)
	}
}

function mousePressed() {
	if(mouseIsPressed) {
		let ind = toIndx(mouseX,mouseY).serialize() 
		if(mouseButton == LEFT) {
			selc.add(ind)
		}else if(mouseButton == CENTER){
			selc.delete(ind)
		}
	}
}

function keyPressed() {
	if(key == 's') {
		saveJSON({'data' : Array.from(selc).map(Pair.from_string)},'./out.json');
	} else if(key == 'c') {
		selc.clear()
	}
}