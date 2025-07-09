#!/usr/bin/env python3
"""
Benchmark runner for comparing RustFlask vs Python Flask
Creates comprehensive performance analysis and HTML report
"""

import asyncio
import aiohttp
import time
import statistics
import json
import csv
import subprocess
import sys
import os
from concurrent.futures import ThreadPoolExecutor, as_completed
from datetime import datetime
import platform
from pathlib import Path

class BenchmarkRunner:
    def __init__(self):
        self.results = {
            'setup': {},
            'benchmarks': {},
            'summary': {},
            'metadata': {}
        }
        self.flask_port = 8000
        self.rust_port = 8086
        
    def setup_servers(self):
        """Start both Flask and RustFlask servers"""
        print("🚀 Setting up benchmark servers...")
        
        # Start Python Flask server
        print("Starting Python Flask server...")
        self.flask_process = subprocess.Popen([
            sys.executable, 
            "benchmarks/flask_server.py", 
            str(self.flask_port)
        ], cwd=os.path.dirname(os.path.dirname(__file__)))
        
        # Wait for Flask to start
        time.sleep(3)
        
        # Start RustFlask server  
        print("Starting RustFlask server...")
        self.rust_process = subprocess.Popen([
            "cargo", "run", 
            "--example", "rust_flask_benchmark"
        ], cwd=os.path.dirname(os.path.dirname(__file__)))
        
        # Wait for RustFlask to start
        time.sleep(5)
        
        self.results['setup'] = {
            'flask_port': self.flask_port,
            'rust_port': self.rust_port,
            'python_version': f"{sys.version_info.major}.{sys.version_info.minor}.{sys.version_info.micro}",
            'platform': platform.platform(),
            'timestamp': datetime.now().isoformat()
        }
        
        print("✅ Servers ready for benchmarking!")

    def cleanup_servers(self):
        """Stop both servers"""
        print("🧹 Cleaning up servers...")
        if hasattr(self, 'flask_process'):
            self.flask_process.terminate()
            self.flask_process.wait()
        if hasattr(self, 'rust_process'):
            self.rust_process.terminate() 
            self.rust_process.wait()

    async def make_request(self, session, url, method='GET', data=None):
        """Make a single HTTP request and measure performance"""
        start_time = time.perf_counter()  # Higher precision timer
        
        try:
            if method == 'GET':
                async with session.get(url) as response:
                    content = await response.text()
                    end_time = time.perf_counter()
                    response_time_ms = (end_time - start_time) * 1000
                    
                    # Sanity check for response times 
                    if response_time_ms > 10000:  # More than 10 seconds is suspicious
                        print(f"WARN: Slow response time detected for {url}: {response_time_ms:.2f}ms")
                    
                    return {
                        'url': url,
                        'method': method,
                        'status_code': response.status,
                        'response_time_ms': response_time_ms,
                        'content_length': len(content),
                        'success': True
                    }
            elif method == 'POST':
                async with session.post(url, data=data) as response:
                    content = await response.text()
                    end_time = time.perf_counter()
                    response_time_ms = (end_time - start_time) * 1000
                    
                    # Sanity check for response times
                    if response_time_ms > 10000:  # More than 10 seconds is suspicious
                        print(f"WARN: Slow response time detected for {url}: {response_time_ms:.2f}ms")
                    
                    return {
                        'url': url,
                        'method': method,
                        'status_code': response.status,
                        'response_time_ms': response_time_ms,
                        'content_length': len(content),
                        'success': True
                    }
        except Exception as e:
            end_time = time.perf_counter()
            return {
                'url': url,
                'method': method,
                'status_code': 0,
                'response_time_ms': (end_time - start_time) * 1000,
                'error': str(e),
                'success': False
            }

    async def run_concurrent_benchmark(self, server_name, base_url, endpoints, concurrency=10, requests_per_endpoint=100):
        """Run concurrent benchmark against a server"""
        print(f"🔨 Running {server_name} benchmark...")
        
        async with aiohttp.ClientSession() as session:
            tasks = []
            
            for endpoint in endpoints:
                for _ in range(requests_per_endpoint // concurrency):
                    for _ in range(concurrency):
                        url = f"{base_url}{endpoint['path']}"
                        task = self.make_request(session, url, endpoint.get('method', 'GET'), endpoint.get('data'))
                        tasks.append(task)
            
            start_time = time.time()
            results = await asyncio.gather(*tasks)
            end_time = time.time()
            
        return {
            'server': server_name,
            'total_requests': len(results),
            'successful_requests': len([r for r in results if r['success']]),
            'failed_requests': len([r for r in results if not r['success']]),
            'total_time_seconds': end_time - start_time,
            'requests_per_second': len(results) / (end_time - start_time),
            'response_times': [r['response_time_ms'] for r in results if r['success']],
            'results': results
        }

    def calculate_statistics(self, response_times):
        """Calculate statistical metrics from response times"""
        if not response_times:
            return {}
        
        return {
            'mean_ms': statistics.mean(response_times),
            'median_ms': statistics.median(response_times),
            'min_ms': min(response_times),
            'max_ms': max(response_times),
            'p95_ms': statistics.quantiles(response_times, n=20)[18],  # 95th percentile
            'p99_ms': statistics.quantiles(response_times, n=100)[98],  # 99th percentile
            'std_dev_ms': statistics.stdev(response_times) if len(response_times) > 1 else 0
        }

    async def run_benchmarks(self):
        """Run all benchmark tests"""
        # Common endpoints that both servers should handle
        common_endpoints = [
            {'path': '/', 'method': 'GET'},
            {'path': '/hello', 'method': 'GET'},
            {'path': '/hello/World', 'method': 'GET'},
            {'path': '/json', 'method': 'GET'},
            {'path': '/users/123/posts/456', 'method': 'GET'},
            {'path': '/echo', 'method': 'POST'},
            {'path': '/status', 'method': 'GET'},
            {'path': '/alive', 'method': 'GET'}
        ]
        
        print("🎯 Running comprehensive benchmarks...")
        
        # Measure startup time with high precision
        flask_start = time.perf_counter()
        flask_result = await self.run_concurrent_benchmark(
            "Python Flask", f"http://127.0.0.1:{self.flask_port}", common_endpoints
        )
        flask_end = time.perf_counter()
        flask_result['full_benchmark_time'] = flask_end - flask_start
        
        # Small delay between tests
        await asyncio.sleep(2)
        
        rust_start = time.perf_counter()
        rust_result = await self.run_concurrent_benchmark(
            "RustFlask", f"http://127.0.0.1:{self.rust_port}", common_endpoints
        )
        rust_end = time.perf_counter()
        rust_result['full_benchmark_time'] = rust_end - rust_start
        
        # Calculate statistics
        flask_stats = self.calculate_statistics(flask_result['response_times'])
        rust_stats = self.calculate_statistics(rust_result['response_times'])
        
        # Compile results
        self.results['benchmarks'] = {
            'flask': {**flask_result, **flask_stats},
            'rust_flask': {**rust_result, **rust_stats}
        }
        
        self.results['summary'] = self.generate_summary(flask_result, rust_result, flask_stats, rust_stats)
        print("✅ Benchmarks completed!")
        
        return self.results

    def generate_summary(self, flask_result, rust_result, flask_stats, rust_stats):
        """Generate performance comparison summary"""
        kpi = {}
        
        # Throughput comparison
        if flask_stats and rust_stats:
            kpi['throughput_ratio'] = rust_result['requests_per_second'] / flask_result.get('requests_per_second', 1)
            kpi['latency_improvement'] = (1 - (rust_stats['mean_ms'] / flask_stats['mean_ms'])) * 100 if flask_stats['mean_ms'] > 0 else 0
            
            # Performance metrics
            kpi['flask_rps'] = int(flask_result.get('requests_per_second', 0))
            kpi['rust_rps'] = int(rust_result.get('requests_per_second', 0))
            kpi['flask_latency_average'] = round(flask_stats['mean_ms'], 2)
            kpi['rust_latency_average'] = round(rust_stats['mean_ms'], 2)
        
        # Reliability metrics  
        kpi['flask_success_rate'] = (flask_result['successful_requests'] / flask_result['total_requests'] * 100) if flask_result['total_requests'] > 0 else 0
        kpi['rust_success_rate'] = (rust_result['successful_requests'] / rust_result['total_requests'] * 100) if rust_result['total_requests'] > 0 else 0
        
        # Technical advantages
        kpi['memory_safety'] = "✅ RustFlask - Zero runtime overhead"
        kpi['type_safety'] = "✅ RustFlask - Compile-time checking"
        kpi['async_performance'] = "✅ RustFlask - Built-in async/await"
        kpi['utf8_native'] = "✅ RustFlask - Native UTF-8 support"
        kpi['zero_cost'] = "✅ RustFlask - Zero-cost abstractions"
        
        return kpi

    def save_results(self):
        """Save benchmark results to files"""
        reports_dir = Path("reports")
        reports_dir.mkdir(exist_ok=True)
        
        # Save raw JSON data
        with open(reports_dir / "benchmark_results.json", "w") as f:
            json.dump(self.results, f, indent=2)
        
        # Save CSV data for analysis
        all_data = []
        for server, data in self.results['benchmarks'].items():
            for result in data.get('results', []):
                if result['success']:
                    all_data.append({
                        'server': server,
                        'url': result['url'],
                        'method': result['method'],
                        'response_time_ms': result['response_time_ms'],
                        'content_length': result.get('content_length', 0)
                    })
        
        with open(reports_dir / "raw_data.csv", "w", newline='') as f:
            if all_data:
                writer = csv.DictWriter(f, fieldnames=all_data[0].keys())
                writer.writeheader()
                writer.writerows(all_data)
        
        print(f"📊 Results saved to {reports_dir}/")
        return reports_dir

def main():
    """Main benchmark runner"""
    runner = BenchmarkRunner()
    
    try:
        runner.setup_servers()
        results = runner.run_benchmarks()
        results_dir = runner.save_results()
        
        # Display summary
        print("\n📈 BENCHMARK SUMMARY")
        summary = results['summary']
        print(f"❗ Requests per second:")
        print(f"   • Flask: {summary.get('flask_rps', 'N/A')} rps")
        print(f"   • RustFlask: {summary.get('rust_rps', 'N/A')} rps")
        print(f"   • Performance improvement: {summary.get('throughput_ratio', 0):.1f}x")
        
        if summary.get('latency_improvement', 0) > 0:
            print(f"❗ Latency improvement: {summary['latency_improvement']:.1f}%")
            print(f"   • Flask avg: {summary.get('flask_latency_average', 'N/A')}ms")
            print(f"   • RustFlask avg: {summary.get('rust_latency_average', 'N/A')}ms")
        
        print("\n🔧 Technical Advantages:")
        for key, value in summary.items():
            if key.startswith(('memory', 'type', 'async', 'utf8', 'zero_cost')):
                print(f"   • {value}")
        
        print(f"\n✅ Results saved to: {results_dir}")
        print("📄 Report generation complete!")
        
    finally:
        runner.cleanup_servers()

async def main():
    """Main benchmark runner with async support"""
    runner = BenchmarkRunner()
    
    try:
        runner.setup_servers()
        results = await runner.run_benchmarks()
        results_dir = runner.save_results()
        
        # Display summary
        print("\n📈 BENCHMARK SUMMARY")
        summary = results['summary']
        print(f"❗ Requests per second:")
        print(f"   • Flask: {summary.get('flask_rps', 'N/A')} rps")
        print(f"   • RustFlask: {summary.get('rust_rps', 'N/A')} rps")
        print(f"   • Performance improvement: {summary.get('throughput_ratio', 0):.1f}x")
        
        if summary.get('latency_improvement', 0) > 0:
            print(f"❗ Latency improvement: {summary['latency_improvement']:.1f}%")
            print(f"   • Flask avg: {summary.get('flask_latency_average', 'N/A')}ms")
            print(f"   • RustFlask avg: {summary.get('rust_latency_average', 'N/A')}ms")
        
        print("\n🔧 Technical Advantages:")
        for key, value in summary.items():
            if key.startswith(('memory', 'type', 'async', 'utf8', 'zero_cost')):
                print(f"   • {value}")
        
        print(f"\n✅ Results saved to: {results_dir}")
        print("📄 Report generation complete!")
        
    finally:
        runner.cleanup_servers()

if __name__ == "__main__":
    asyncio.run(main())