import base64
import requests


base = base64.b64encode(open("./test-data/test-image.png", "rb").read()).decode("ascii")
# headers = {'Content-type': 'application/json'}

# base = "foobar"

res = requests.post("http://localhost:3000/api/graph", json={'funcs': [{'func': 'f(x) = 2x^2', 'color': '#00ffff'}, {'func': 'g(x) = 4x + 5', 'color': '#EEaa22'}], 'range': [-10, 10], 'image': base})

print(str(res))
# print(str(res.content))
with open("graph.server.png", "wb") as f:
    f.write(res.content)

