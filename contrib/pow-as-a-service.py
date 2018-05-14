#!/usr/bin/env python
"""
PoW as a service
"""
__author__ = "domenukk"
import subprocess
from flask import Flask

app = Flask(__name__)

def pexec(*args):
   return subprocess.Popen(args, stdout=subprocess.PIPE).communicate()[0].rstrip()

def calculate(challenge, difficulty):
   return pexec("./defcon26-pow", challenge, difficulty).split('Solution: ')[1].split(' ->')[0]


@app.route('/powpowP0W/<challenge>')
def get_pow(challenge):
   """
   Will calculate the proof of work using multicore rust
   Example localhost:9044/powpowP0W/g6tCf6oicW
   """
   return calculate(challenge, "27")

@app.route('/powpowP0W/<challenge>/<difficulty>')
def get_pow_pow(challenge, difficulty):
   """
   Will calculate the proof of work using multicore rust, but this time not hard code the difficulty to 27
   Example localhost:9044/powpowP0W/g6tCf6oicW/26
   """
   return calculate(challenge, difficulty)

if __name__ == '__main__':
   print("Use route /powpowP0W/<challenge>(/<difficulty>)")
   app.run(host="0.0.0.0", port=9044)
