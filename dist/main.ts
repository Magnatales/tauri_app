const btn = document.querySelector('#btn')
const btn2 = document.querySelector('#btn2')

// @ts-ignore
const invoke = window.__TAURI__.invoke

btn.addEventListener('click',() =>{
    alert('Hello World')
})

btn2.addEventListener('click',() =>{
    invoke('create_hero', {name: "Magna from button", hp: "20000"}).then((response) => console.log(response))
})