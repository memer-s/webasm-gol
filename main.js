import init, {GOL} from "./pkg/gol.js";

let golSize = {x: 100, y: 100}

/** @type {HTMLCanvasElement} */
let c = document.getElementById("c")


let ctx = c.getContext("2d")

c.height = 800;
c.width = 800;

await init()

window.GOL = GOL

let g = GOL.new(golSize.x, golSize.y);
window.gol = g;


window.print = () => {
	return JSON.parse(g.getState().split('gameState: ')[1].split(" }")[0])
}

document.getElementById("step").addEventListener("click", () => {
	render()
	window.gol.step()
})

let render = () => {
	const stuff = window.print();
	ctx.clearRect(0,0,c.width,c.height)
	for(let i = 0; i < golSize.y; i++) {
		for(let j = 0; j < golSize.x; j++) {
			if(stuff[j][i] == 0) {
				ctx.fillStyle = 'white';
				ctx.fillRect(i*(c.height/golSize.y), j*(c.width/golSize.x), c.height/golSize.y, c.width/golSize.x)
			}
			else {
				ctx.fillStyle = 'black';
				ctx.fillRect(i*(c.height/golSize.y), j*(c.width/golSize.x), c.height/golSize.y, c.width/golSize.x)
			}
		}
	}
};

c.addEventListener("click", (d) => {
	let box = c.getBoundingClientRect()
	window.gol.setPixel(Math.floor((d.x-box.left)/(golSize.x/12)), Math.floor(d.y-box.top)/(golSize.y/12))

	render()
})