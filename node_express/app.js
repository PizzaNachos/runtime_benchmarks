const express = require('express')
const app = express()
const port = 3000

app.get('/sleep/:delay', async (req, res) => {
    let delay = req.params.delay
    await new Promise(r => setTimeout(r, delay));
    res.send(delay)
})
app.get('/', async (req, res) => {
    res.send("Hello! - Express")
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})