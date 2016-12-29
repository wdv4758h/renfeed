========================================================================
RenFeed - collect RSS/Atom into one site
========================================================================


Inspired by `Planet Rust <http://www.planet-rust.com/>`_

Screenshot:

.. image:: https://cloud.githubusercontent.com/assets/2716047/21541474/ac17076a-cdf1-11e6-87ff-e3c196b7ecaa.png


Demo Site: https://wdv4758h.github.io/renfeed/


.. contents:: Table of Contents


Usage
========================================

.. code-block:: sh

    git clone https://github.com/wdv4758h/renfeed
    cd renfeed/renfeed_gen_site
    cargo build --release
    cd ../
    ./target/release/renfeed_gen_site
    cd share/
    python -m http.server   # start web server



Special Thanks
========================================

* `serde <https://github.com/serde-rs/serde>`_ and `serde-yaml <https://github.com/dtolnay/serde-yaml>`_ for parsing YAML configs
* `hyper <https://github.com/hyperium/hyper>`_ for handling HTTP requests
* `rss <https://github.com/rust-syndication/rss>`_ for parsing RSS feeds
* `rust-atom <https://github.com/vtduncan/rust-atom>`_ for parsing Atom feeds
* `tera <https://github.com/Keats/tera>`_ for generating HTML
* `clap-rs <https://github.com/kbknapp/clap-rs>`_ for parsing CLI arguments
* `Rust Team <https://www.rust-lang.org/team.html>`_
* and every project I've used



License
========================================

RenFeed is licensed under the Apache-2.0 License - see the ``LICENSE`` file for details
