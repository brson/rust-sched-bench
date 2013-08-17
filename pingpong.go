package main

func ping_pong_bench(n uint, m uint) {
	 endc := make(chan byte)

	 for i := uint(0); i < m; i++ {
	 	 run_pair(n, endc)
	 }

	 for i := uint(0); i < m; i++ {
	 	 <- endc
		 <- endc
	 }
}

func run_pair(n uint, endc chan(byte)) {
	 ca := make(chan byte)
	 cb := make(chan byte)

	 go func() {
	     for i := uint(0); i < n; i++ {
		     ca <- byte(0)
			 <- cb
		 }

		 endc <- byte(0)
     }()

	 go func() {
	  	 for i := uint(0); i < n; i++ {
		     <- ca
			 cb <- byte(0)
		 }

		 endc <- byte(0)
	 }()
}

func main() {
	 ping_pong_bench(100000, 16)
}

