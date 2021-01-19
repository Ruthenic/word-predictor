from flask import Flask, render_template, redirect, url_for,request
from flask import make_response
import subprocess
import importlib  
produce = importlib.import_module("produce-web")

app = Flask(__name__)
result = 'ERROR: GENERATION FAILED. PLEASE REFRESH'
@app.route("/")
def index():
    return render_template('input.html')

@app.route('/', methods=['POST'])
def index_post():
    word = request.form['word']
    #subprocess.call('./produce.py ' + word, shell=True)
    result = produce.generator.gen(word,50)
    #with open('result.txt') as f:
    #    result = ''
    #    for line in f:
    #        result = result + '\n' + line
    return result
    
#@app.route("/result", methods=['POST'])
#def result():
#    if request.method == 'POST':
#        text = request.form['text']
#        processed_text = text.upper()
#        return processed_text
