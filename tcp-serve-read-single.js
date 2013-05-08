var net = require('net');

var HOST = '127.0.0.1';
var PORT = 9001;
var msg_piece = '0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmno';
var msg = "";
for (var i = 0; i < 1024; i++) {
    msg += msg_piece;
}

var msgbuf = new Buffer(msg, 'ascii');

var NUM_BYTES = 1024 * 64 * 1024;

var server = net.createServer();
server.on('connection', function(sock) {

    var bytes = 0;

    sock.setEncoding('ascii');
    sock.on('data', function(data) {
        bytes += data.length;
        if (bytes >= NUM_BYTES) {
            sock.destroy();
            server.close();
            console.log("read " + bytes + " bytes");
        }
    })

});
server.listen(PORT, HOST);
