import sys
import os
import subprocess

print(sys.argv)
exec_path = sys.argv[1]
testsuite_path = sys.argv[2] if len(sys.argv) > 2 else os.getcwd()

def run_exec(path, input=None):
    return subprocess.run([exec_path, path], input=input, stdout=subprocess.PIPE, stderr=subprocess.PIPE, universal_newlines=True)

def handle_suite(path, run_test):
    names = set(os.path.splitext(f)[0] for f in os.listdir(path) if os.path.isfile(os.path.join(path, f)))
    names = sorted(list(names))

    for name in names:
        run_test(name)

def handle_good_suite(next_path):
    good_path = os.path.join(testsuite_path, next_path)

    def run_good_test(name):
        code_path = os.path.join(good_path, name + ".jl")
        output_path = os.path.join(good_path, name + ".output")
        input_path = os.path.join(good_path, name + ".input")

        with open(output_path) as f:
            expected_output = f.read()

        try:
            with open(input_path) as f:
                given_input = f.read()
        except:
            given_input = None

        res = run_exec(code_path, given_input)
        print("Test {} : {}".format(name, "OK" if expected_output == res.stdout else "FAIL"))

    handle_suite(good_path, run_good_test)

def handle_bad_suite():
    bad_path = os.path.join(testsuite_path, "bad")

    def run_bad_test(name):
        code_path = os.path.join(bad_path, name + ".jl")

        res = run_exec(code_path)
        print("Test {} : {} (exit code {})".format(name, "OK" if res.returncode == 1 else "FAIL", res.returncode))
    handle_suite(bad_path, run_bad_test)

handle_good_suite("good")
handle_good_suite("extensions/arrays1")
handle_good_suite("extensions/arrays2")
handle_bad_suite()