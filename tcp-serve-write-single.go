package main

import "net"
import "fmt"

func main() {

    var BUF_SIZE = 64 * 1024;
    var NUM_WRITES = 64 * 1024;

    var buf = make([]byte, BUF_SIZE);

    for i := 0; i < BUF_SIZE; i++ {
        buf[i] = (byte)(i % 64 + 48)
    }

    ln, _ := net.Listen("tcp", "localhost:9001")

    conn, _ := ln.Accept()

    for i := 0; i < NUM_WRITES; i++ {
        conn.Write(buf);
    }

    fmt.Println("wrote %u bytes", NUM_WRITES * BUF_SIZE)
}
