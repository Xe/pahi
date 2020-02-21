package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
)

var (
	testDataFname   = flag.String("data", PathToTestdata, "path to test data file")
	dhallToJSONPath = flag.String("dhall-to-json-path", which("dhall-to-json"), "path to dhall-to-json")
	pathToWasmFiles = flag.String("wasm-path", PathToWasmFiles, "where the wasm files to test are located")
)

func which(name string) string {
	pth, err := exec.LookPath(name)
	if err != nil /* fundamental assertion error */ {
		panic(err)
	}

	return pth
}

type TestSuite struct {
	Cases []struct {
		ExitsWith   int    `json:"exitsWith"`
		Fname       string `json:"fname"`
		Interpreter string `json:"interpreter"`
	} `json:"cases"`
	Env []string `json:"env"`
}

func main() {
	os.Setenv("PATH", "../result/bin:"+os.Getenv("PATH"))

	flag.Parse()

	cmd := exec.Command(*dhallToJSONPath, "--file", *testDataFname)
	outp, err := cmd.Output()
	if err != nil /* fundamental assertion error */ {
		log.Fatal(err)
	}

	var suite TestSuite
	err = json.Unmarshal(outp, &suite)
	if err != nil /* fundamental assertion error */ {
		log.Fatal(err)
	}

	for i, cs := range suite.Cases {
		i++
		outBuf := bytes.NewBuffer(nil)
		errBuf := bytes.NewBuffer(nil)
		inBuf := bytes.NewBuffer([]byte("hahaha it's is an laughter image"))

		cmd := exec.Command(which(cs.Interpreter), filepath.Join(*pathToWasmFiles, cs.Fname))
		cmd.Stdout = outBuf
		cmd.Stderr = errBuf
		cmd.Stdin = inBuf
		cmd.Env = suite.Env
		failed := false

		err := cmd.Run()

		if err != nil {
			failed = true
			fmt.Printf("%s running %s\n", cs.Interpreter, cs.Fname)
			fmt.Printf("Stdout: \n%s\n", outBuf.String())
			fmt.Printf("Stderr: \n%s\n", errBuf.String())
		}

		if cmd.ProcessState.Exited() && cmd.ProcessState.ExitCode() != cs.ExitsWith {
			failed = true
			fmt.Printf("wanted exit code %d, got: %d\n", cs.ExitsWith, cmd.ProcessState.ExitCode())
		}

		msg := "OK"
		if failed {
			msg = "NOT OK"
		}

		fmt.Printf("%d %s %s %s\n", i, msg, cs.Interpreter, cs.Fname)
	}
}
