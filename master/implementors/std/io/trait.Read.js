(function() {var implementors = {};
implementors["bytes"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='bytes/struct.RingBuf.html' title='bytes::RingBuf'>RingBuf</a>","impl&lt;T: <a class='trait' href='bytes/buf/trait.Buf.html' title='bytes::buf::Buf'>Buf</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='bytes/struct.Take.html' title='bytes::Take'>Take</a>&lt;T&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='bytes/struct.ByteBuf.html' title='bytes::ByteBuf'>ByteBuf</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='bytes/struct.ROByteBuf.html' title='bytes::ROByteBuf'>ROByteBuf</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='bytes/struct.RopeBuf.html' title='bytes::RopeBuf'>RopeBuf</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/alloc/boxed/struct.Box.html' title='alloc::boxed::Box'>Box</a>&lt;<a class='trait' href='bytes/buf/trait.Buf.html' title='bytes::buf::Buf'>Buf</a> + 'static&gt;",];implementors["sodiumoxide_extras"] = [];implementors["openssl"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='openssl/bio/struct.MemBio.html' title='openssl::bio::MemBio'>MemBio</a>","impl&lt;S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='openssl/ssl/struct.SslStream.html' title='openssl::ssl::SslStream'>SslStream</a>&lt;S&gt;","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='openssl/ssl/enum.MaybeSslStream.html' title='openssl::ssl::MaybeSslStream'>MaybeSslStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>",];implementors["mio"] = ["impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/tcp/struct.TcpStream.html' title='mio::tcp::TcpStream'>TcpStream</a>","impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/unix/struct.UnixStream.html' title='mio::unix::UnixStream'>UnixStream</a>","impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/unix/struct.PipeReader.html' title='mio::unix::PipeReader'>PipeReader</a>","impl&lt;'a&gt; <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for &amp;'a <a class='struct' href='mio/unix/struct.PipeReader.html' title='mio::unix::PipeReader'>PipeReader</a>","impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/struct.Io.html' title='mio::Io'>Io</a>","impl&lt;'a&gt; <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for &amp;'a <a class='struct' href='mio/struct.Io.html' title='mio::Io'>Io</a>",];implementors["tmp_mio"] = ["impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tmp_mio/tcp/struct.TcpStream.html' title='tmp_mio::tcp::TcpStream'>TcpStream</a>","impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tmp_mio/unix/struct.UnixStream.html' title='tmp_mio::unix::UnixStream'>UnixStream</a>","impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tmp_mio/unix/struct.PipeReader.html' title='tmp_mio::unix::PipeReader'>PipeReader</a>","impl&lt;'a&gt; <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for &amp;'a <a class='struct' href='tmp_mio/unix/struct.PipeReader.html' title='tmp_mio::unix::PipeReader'>PipeReader</a>","impl <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tmp_mio/struct.Io.html' title='tmp_mio::Io'>Io</a>","impl&lt;'a&gt; <a class='trait' href='std/io/trait.Read.html' title='std::io::Read'>Read</a> for &amp;'a <a class='struct' href='tmp_mio/struct.Io.html' title='tmp_mio::Io'>Io</a>",];implementors["hyper"] = ["impl&lt;S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/client/pool/struct.PooledStream.html' title='hyper::client::pool::PooledStream'>PooledStream</a>&lt;S&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/client/response/struct.Response.html' title='hyper::client::response::Response'>Response</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='hyper/client/enum.Body.html' title='hyper::client::Body'>Body</a>&lt;'a&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/http/h1/struct.Http11Message.html' title='hyper::http::h1::Http11Message'>Http11Message</a>","impl&lt;R: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='hyper/http/h1/enum.HttpReader.html' title='hyper::http::h1::HttpReader'>HttpReader</a>&lt;R&gt;","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/http/h2/struct.Http2Message.html' title='hyper::http::h2::Http2Message'>Http2Message</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/http/h2/trait.CloneableStream.html' title='hyper::http::h2::CloneableStream'>CloneableStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/net/struct.HttpStream.html' title='hyper::net::HttpStream'>HttpStream</a>","impl&lt;S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='hyper/net/enum.HttpsStream.html' title='hyper::net::HttpsStream'>HttpsStream</a>&lt;S&gt;","impl&lt;'a, 'b&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/server/request/struct.Request.html' title='hyper::server::request::Request'>Request</a>&lt;'a, 'b&gt;",];implementors["ws"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/net/tcp/struct.TcpStream.html' title='mio::net::tcp::TcpStream'>TcpStream</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/net/unix/struct.UnixStream.html' title='mio::net::unix::UnixStream'>UnixStream</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/net/unix/struct.PipeReader.html' title='mio::net::unix::PipeReader'>PipeReader</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for &amp;'a <a class='struct' href='mio/net/unix/struct.PipeReader.html' title='mio::net::unix::PipeReader'>PipeReader</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='mio/sys/unix/io/struct.Io.html' title='mio::sys::unix::io::Io'>Io</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for &amp;'a <a class='struct' href='mio/sys/unix/io/struct.Io.html' title='mio::sys::unix::io::Io'>Io</a>",];implementors["maidsafe_utilities"] = [];implementors["igd"] = ["impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/client/pool/struct.PooledStream.html' title='hyper::client::pool::PooledStream'>PooledStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/client/response/struct.Response.html' title='hyper::client::response::Response'>Response</a>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='hyper/client/enum.Body.html' title='hyper::client::Body'>Body</a>&lt;'a&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/http/h1/struct.Http11Message.html' title='hyper::http::h1::Http11Message'>Http11Message</a>","impl&lt;R&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='hyper/http/h1/enum.HttpReader.html' title='hyper::http::h1::HttpReader'>HttpReader</a>&lt;R&gt; <span class='where'>where R: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a></span>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/http/h2/struct.Http2Message.html' title='hyper::http::h2::Http2Message'>Http2Message</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/http/h2/trait.CloneableStream.html' title='hyper::http::h2::CloneableStream'>CloneableStream</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/net/struct.HttpStream.html' title='hyper::net::HttpStream'>HttpStream</a>","impl&lt;S&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='enum' href='hyper/net/enum.HttpsStream.html' title='hyper::net::HttpsStream'>HttpsStream</a>&lt;S&gt; <span class='where'>where S: <a class='trait' href='hyper/net/trait.NetworkStream.html' title='hyper::net::NetworkStream'>NetworkStream</a></span>","impl&lt;'a, 'b&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='hyper/server/request/struct.Request.html' title='hyper::server::request::Request'>Request</a>&lt;'a, 'b&gt;",];implementors["crust"] = [];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
