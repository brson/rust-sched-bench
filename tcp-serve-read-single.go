package main

import "net"
import "fmt"

func main() {

	var BUF_SIZE = 64 * 1024;
	var NUM_BYTES = 1024 * BUF_SIZE;

	var buf = make([]byte, BUF_SIZE);

	ln, _ := net.Listen("tcp", "localhost:9001")

	conn, _ := ln.Accept()

	var bytes = 0;

	for {
		read, _ := conn.Read(buf);
		bytes += read;
		if (bytes >= NUM_BYTES) {
			break;
		}
	}

	fmt.Println("read %u bytes", bytes)
}
