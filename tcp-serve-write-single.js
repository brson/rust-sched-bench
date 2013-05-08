var net = require('net');

var HOST = '127.0.0.1';
var PORT = 9001;
var msg_piece = '0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmno';
var msg = "";
for (var i = 0; i < 1024; i++) {
    msg += msg_piece;
}

var msgbuf = new Buffer(msg, 'ascii');

var NUM_WRITES = 64 * 1024;

var server = net.createServer();
server.on('connection', function(sock) {

    writemsg(0, sock);

    function writemsg(i, sock) {
	if (i == NUM_WRITES) {
	    console.log("wrote " + msgbuf.length * NUM_WRITES + " bytes");
	    sock.destroy();
	    server.close(0);
	    return;
	}

	sock.write(msgbuf, function() {
	    writemsg(i + 1, sock);
	});
    }
});
server.listen(PORT, HOST);
