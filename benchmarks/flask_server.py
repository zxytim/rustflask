#!/usr/bin/env python3
"""
Python Flask benchmark server for comparison with RustFlask
"""

from flask import Flask, request, jsonify, Response
from datetime import datetime
import json
import time
import sys

app = Flask(__name__)

@app.route('/')
def home():
    return '''
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Flask Comparison Server</title>
    </head>
    <body>
        <h1>Flask Benchmark Server</h1>
        <p>This is the Python Flask server for benchmarking against RustFlask</p>
    </body>
    </html>
    '''

@app.route('/hello')
def hello():
    return "Hello from Python Flask! \U0001F60A"

@app.route('/hello/<name>')
def hello_name(name):
    return f"Hello, {name}! Welcome to Flask! \U0001F680\n\nTe: \u4f60\u597d\u4e16\u754c (Hello World in Chinese)"

@app.route('/json')
def json_response():
    return jsonify({
        "framework": "Flask",
        "version": "2.0+",
        "features": {
            "http_methods": True,
            "url_parameters": True,
            "json_serialization": True,
            "template_rendering": True,
            "extensions": True
        },
        "timestamp": datetime.utcnow().isoformat(),
        "success": True,
        "language": "Python",
        "framework_type": "Web Framework",
        "performance": "interpreted"
    })

@app.route('/users/<id>/posts/<post_id>')
def multiple_params(id, post_id):
    return jsonify({
        "message": "Multiple URL parameters captured!",
        "user_id": id,
        "post_id": post_id,
        "path_info": f"User {id} -> Post {post_id}",
        "framework": "Flask",
        "demonstration": "url_parameter_extraction",
        "features": ["parameters", "routing"]
    })

@app.route('/echo', methods=['POST'])
def echo():
    return jsonify({
        "message": "POST request received successfully!",
        "method": "POST",
        "headers": dict(request.headers),
        "endpoint": "/echo",
        "framework": "Flask",
        "timestamp": datetime.utcnow().isoformat()
    })

@app.route('/status')
def status():
    return jsonify({
        "status": "running",
        "framework": "Flask",
        "version": "2.0+",
        "encoding": "UTF-8",
        "features": ["sync", "methods", "parameters", "json"],
        "language": "Python",
        "memory_management": "garbage_collected",
        "health": "operational"
    })

@app.route('/alive')
def alive():
    return jsonify({
        "status": "alive",
        "message": "Flask server is running!",
        "timestamp": datetime.utcnow().isoformat(),
        "framework": "Flask", 
        "version": "2.0+",
        "uptime": "running"
    })

if __name__ == '__main__':
    # Run on different ports for testing
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    app.run(host='127.0.0.1', port=port, debug=True, threaded=True)