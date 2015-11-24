var net = require('net');
var client = net.connect({port: 6000},
    function() { //'connect' listener
        console.log('connected to server!');
        var data = new Buffer(10, 2, 2, 1);
        client.write(data);

        setTimeout(function() { client.write(data); }, 1000);
    });
client.on('data', function(data) {
    console.log(data.toString());
    //client.end();
});
client.on('end', function() {
    console.log('disconnected from server');
});
