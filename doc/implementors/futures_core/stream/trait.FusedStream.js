(function() {var implementors = {};
implementors["futures_channel"] = [{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.Receiver.html\" title=\"struct futures_channel::mpsc::Receiver\">Receiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::Receiver"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"futures_channel/mpsc/struct.UnboundedReceiver.html\" title=\"struct futures_channel::mpsc::UnboundedReceiver\">UnboundedReceiver</a>&lt;T&gt;","synthetic":false,"types":["futures_channel::mpsc::UnboundedReceiver"]}];
implementors["futures_core"] = [];
implementors["mysql_async"] = [{"text":"impl&lt;'r, 'a: 'r, 't: 'a, T, P&gt; <a class=\"trait\" href=\"futures_core/stream/trait.FusedStream.html\" title=\"trait futures_core::stream::FusedStream\">FusedStream</a> for <a class=\"struct\" href=\"mysql_async/struct.ResultSetStream.html\" title=\"struct mysql_async::ResultSetStream\">ResultSetStream</a>&lt;'r, 'a, 't, T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"mysql_async/prelude/trait.Protocol.html\" title=\"trait mysql_async::prelude::Protocol\">Protocol</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"mysql_async/prelude/trait.FromRow.html\" title=\"trait mysql_async::prelude::FromRow\">FromRow</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.61.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,&nbsp;</span>","synthetic":false,"types":["mysql_async::queryable::query_result::result_set_stream::ResultSetStream"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()