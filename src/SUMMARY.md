# Summary

[Hyperware](./hyperware_overview.md)
- [Hyperware OS](./getting_started/getting_started.md)
  - [Quick Start](./getting_started/quick_start.md)
    - [Introduction](./getting_started/intro.md)
    - [Hypermap and HNS](./getting_started/hypermap.md)
    - [Design Philosophy](./getting_started/design_philosophy.md)
    - [Installation](./getting_started/install.md)
    - [Join the Network](./getting_started/login.md)
  - [System Components](./system/system_components.md)
    - [Processes](./system/processes_overview.md)
      - [Process Semantics](./system/process/processes.md)
      - [Capability-Based Security](./system/process/capabilities.md)
      - [Startup, Spindown, and Crashes](./system/process/startup.md)
      - [Extensions](./system/process/extensions.md)
      - [WIT APIs](./system/process/wit_apis.md)
    - [Networking Protocol](./system/networking_protocol.md)
    - [HTTP Server & Client](./system/http_server_and_client.md)
    - [Read+Write to Chain](./system/read_and_write_to_chain.md)
    - [Files](./system/files.md)
    - [Databases](./system/databases.md)
    - [Terminal](./system/terminal.md)
  - [Process Standard Library](./process_stdlib/overview.md)
  - [Kit: Development Toolkit](./kit/kit-dev-toolkit.md)
    - [Installation](./kit/install.md)
    - [`boot-fake-node`](./kit/boot-fake-node.md)
    - [`new`](./kit/new.md)
    - [`build`](./kit/build.md)
    - [`start-package`](./kit/start-package.md)
    - [`publish`](./kit/publish.md)
    - [`build-start-package`](./kit/build-start-package.md)
    - [`remove-package`](./kit/remove-package.md)
    - [`chain`](./kit/chain.md)
    - [`dev-ui`](./kit/dev-ui.md)
    - [`inject-message`](./kit/inject-message.md)
    - [`run-tests`](./kit/run-tests.md)
    - [`connect`](./kit/connect.md)
    - [`reset-cache`](./kit/reset-cache.md)
    - [`boot-real-node`](./kit/boot-real-node.md)
    - [`view-api`](./kit/view-api.md)
  - [My First Hyperware Application](./my_first_app/build_and_deploy_an_app.md)
    - [Environment Setup](./my_first_app/chapter_1.md)
    - [Sending and Responding to a Message](./my_first_app/chapter_2.md)
    - [Messaging with More Complex Data Types](./my_first_app/chapter_3.md)
    - [Frontend Time](./my_first_app/chapter_4.md)
    - [Sharing with the World](./my_first_app/chapter_5.md)
  - [In-Depth Guide: Chess App](./chess_app/chess_app.md)
    - [Environment Setup](./chess_app/setup.md)
    - [Chess Engine](./chess_app/chess_engine.md)
    - [Adding a Frontend](./chess_app/frontend.md)
    - [Putting Everything Together](./chess_app/putting_everything_together.md)
    - [Extension: Chat](./chess_app/chat.md)
  - [Cookbook (Handy Recipes)](./cookbook/cookbook.md)
    - [Saving State](./cookbook/save_state.md)
    - [Managing Child Processes](./cookbook/manage_child_processes.md)
    - [Publishing a Website or Web App](./cookbook/publish_to_web.md)
    - [Simple File Transfer Guide](./cookbook/file_transfer.md)
    - [Intro to Web UI with File Transfer](./cookbook/file_transfer_ui.md)
    - [Writing and Running Scripts](./cookbook/writing_scripts.md)
    - [Reading Data from ETH](./cookbook/reading_data_from_eth.md)
    - [Writing Data to ETH](./cookbook/writing_data_to_eth.md)
    - [Creating and Using Capabilities](./cookbook/creating_and_using_capabilities.md)
    - [Managing Contacts](./cookbook/managing_contacts.md)
    - [Use ZK proofs with SP1](./cookbook/zk_with_sp1.md)
    - [Talking to the Outside World](./cookbook/talking_to_the_outside_world.md)
    - [Exporting & Importing Package APIs](./cookbook/package_apis.md)
    - [Exporting Workers in Package APIs](./cookbook/package_apis_workers.md)
  - [API Reference](./apis/api_reference.md)
    - [ETH Provider API](./apis/eth_provider.md)
    - [Frontend/UI Development](./apis/frontend_development.md)
    - [HTTP API](./apis/http_authentication.md)
    - [HTTP Client API](./apis/http_client.md)
    - [HTTP Server API](./apis/http_server.md)
    - [Kernel API](./apis/kernel.md)
    - [`hyperware.wit`](./apis/hyperware_wit.md)
    - [KV API](./apis/kv.md)
    - [Net API](./apis/net.md)
    - [SQLite API](./apis/sqlite.md)
    - [Terminal API](./apis/terminal.md)
    - [Timer API](./apis/timer.md)
    - [VFS API](./apis/vfs.md)
    - [WebSocket API](./apis/websocket.md)
  - [Hosted Nodes User Guide](./hosted-nodes.md)
  - [Audits and Security](./audits-and-security.md)
  - [Glossary](./glossary.md)
- [Hypergrid](./hypergrid_overview.md)
  - [Running an Operator Node](./hypergrid/running_operator_node.md)
  - [Running a Provider Node](./hypergrid/running_provider_node.md)
