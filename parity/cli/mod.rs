// Copyright 2015-2017 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use]
mod usage;
use dir;

usage! {
	{
		// Flags
		// -- Legacy Options
		flag_geth: bool,
		flag_testnet: bool,
		flag_import_geth_keys: bool,
		flag_datadir: Option<String>,
		flag_networkid: Option<u64>,
		flag_peers: Option<u16>,
		flag_nodekey: Option<String>,
		flag_nodiscover: bool,
		flag_jsonrpc: bool,
		flag_jsonrpc_off: bool,
		flag_webapp: bool,
		flag_dapps_off: bool,
		flag_rpc: bool,
		flag_rpcaddr: Option<String>,
		flag_rpcport: Option<u16>,
		flag_rpcapi: Option<String>,
		flag_rpccorsdomain: Option<String>,
		flag_ipcdisable: bool,
		flag_ipc_off: bool,
		flag_ipcapi: Option<String>,
		flag_ipcpath: Option<String>,
		flag_gasprice: Option<String>,
		flag_etherbase: Option<String>,
		flag_extradata: Option<String>,
		flag_cache: Option<u32>,

		// I guess the following are options that can only be set from the CLI ?
		flag_no_config: bool,
//		"--no-config
			//'Don't load a configuration file.'",

	}
	{ // OLD BLOCK, WIP TO MOVE IT TO ARGS OR FLAGS // this block will go away soon
		// -- Legacy Options supported in configs
		flag_dapps_port: Option<u16> = None, or |c: &Config| otry!(c.dapps).port.clone().map(Some),
		flag_dapps_interface: Option<String> = None, or |c: &Config| otry!(c.dapps).interface.clone().map(Some),
		flag_dapps_hosts: Option<String> = None, or |c: &Config| otry!(c.dapps).hosts.as_ref().map(|vec| Some(vec.join(","))),
		flag_dapps_cors: Option<String> = None, or |c: &Config| otry!(c.dapps).cors.clone().map(Some),
		flag_dapps_user: Option<String> = None, or |c: &Config| otry!(c.dapps).user.clone().map(Some),
		flag_dapps_pass: Option<String> = None, or |c: &Config| otry!(c.dapps).pass.clone().map(Some),
		flag_dapps_apis_all: Option<bool> = None, or |_| None,
	}
	{
		// Values with optional default value. (@TODO prune? merge with the other block?)
		flag_base_path: Option<String>, display dir::default_data_path(), or |c: &Config| otry!(c.parity).base_path.clone().map(Some),
		flag_db_path: Option<String>, display dir::CHAINS_PATH, or |c: &Config| otry!(c.parity).db_path.clone().map(Some),
		flag_warp: Option<bool>, display true, or |c: &Config| Some(otry!(c.network).warp.clone()),
	}
	{
		// CLI subcommands
		// Subcommands/sub-subcommands must start with cmd_
		// Arguments must start with arg_

		CMD cmd_wallet
		{
			CMD cmd_wallet_import
			{
				ARG arg_wallet_import_path: String, |arg| Arg::required(arg,true).index(3),
				ARG arg_wallet_import_password: String, |arg| Arg::required(arg,true).value_name("FILE"),
			}
		}

		CMD cmd_account
		{
			CMD cmd_account_new {}

			CMD cmd_account_list {}

			CMD cmd_account_import
			{
				ARGM arg_account_import_path: String, |arg| Arg::required(arg,true).index(3),
			}
		}

		CMD cmd_dapp
		{
			ARG arg_dapp_path: String, |arg| Arg::index(arg,2).required(true),
		}
		
		CMD cmd_daemon
		{
			ARG arg_daemon_pid_file: String, |arg| Arg::index(arg,2).required(true),
		}

		CMD cmd_export // evtl: |&sc: SubCommand| sc.help("blablabla")
		{
			CMD cmd_export_blocks
			{
				ARG arg_export_blocks_file: String, |arg| Arg::index(arg,3),
			}

			CMD cmd_export_state
			{
				ARG arg_export_state_file: String, |arg| Arg::index(arg,3),
			}
		}
		
		CMD cmd_import
		{
			ARG arg_import_file: String, |arg| Arg::index(arg,2),
		}

		CMD cmd_signer
		{
			CMD cmd_signer_new_token {}

			CMD cmd_signer_list {}

			CMD cmd_signer_sign
			{
				ARG arg_signer_sign_id: usize, |arg| Arg::index(arg,3),
				ARG arg_signer_sign_password: String, |arg| Arg::value_name(arg,"FILE"),
			}

			CMD cmd_signer_reject
			{
				ARG arg_signer_reject_id: usize, |arg| Arg::index(arg,3),
			}
		}

		CMD cmd_snapshot
		{
			ARG arg_snapshot_file: String, |arg| Arg::required(arg,true).index(2),
		}

		CMD cmd_restore
		{
			ARG arg_restore_file: String, |arg| Arg::index(arg,2),
		}

		CMD cmd_ui {}

		CMD cmd_tools
		{
			CMD cmd_tools_hash
			{
				ARG arg_tools_hash_file: String, |arg| Arg::required(arg,true).index(3),
			}
		}
		
		CMD cmd_db
		{
			CMD cmd_db_kill {}
		}
	}
	{
		["Operating Options"]
			FLAG flag_public_node: bool = false, or |c: &Config| otry!(c.parity).public_node.clone(),
			"--public-node
				'Start Parity as a public web server. Account storage and transaction signing will be delegated to the UI.'",
			
			FLAG flag_no_download: bool = false, or |c: &Config| otry!(c.parity).no_download.clone(),
			"--no-download
				'Normally new releases will be downloaded ready for updating. This disables it. Not recommended.'",

			FLAG flag_no_consensus: bool = false, or |c: &Config| otry!(c.parity).no_consensus.clone(),
			"--no-consensus
				'Force the binary to run even if there are known issues regarding consensus. Not recommended.'",

			FLAG flag_light: bool = false, or |c: &Config| otry!(c.parity).light,
			"--light
				'Experimental: run in light client mode. Light clients synchronize a bare minimum of data and fetch necessary data on-demand from the network. Much lower in storage, potentially higher in bandwidth. Has no effect with subcommands.'",

			ARG arg_mode: String = "last", or |c: &Config| otry!(c.parity).mode.clone(),
			"--mode MODE\
				'Set the operating mode. MODE can be one of:\
					last - Uses the last-used mode, active if none.\
					active - Parity continuously syncs the chain.\
					passive - Parity syncs initially, then sleeps and wakes regularly to resync.\
					dark - Parity syncs only when the RPC is active.\
					offline - Parity doesn't sync.'",

			ARG arg_mode_timeout: u64 = 300u64, or |c: &Config| otry!(c.parity).mode_timeout.clone(),
			"--mode-timeout SECS
				'Specify the number of seconds before inactivity timeout occurs when mode is dark or passive'",

			ARG arg_mode_alarm: u64 = 3600u64, or |c: &Config| otry!(c.parity).mode_alarm.clone(),
			"--mode-alarm SECS
				'Specify the number of seconds before auto sleep reawake timeout occurs when mode is passive'",

			ARG arg_auto_update: String = "critical", or |c: &Config| otry!(c.parity).auto_update.clone(),
			"--auto-update SET
				'Set a releases set to automatically update and install.
					all - All updates in the our release track.
					critical - Only consensus/security updates.
					none - No updates will be auto-installed.'",

			ARG arg_release_track: String = "current", or |c: &Config| otry!(c.parity).release_track.clone(),
			"--release-track TRACK
				'Set which release track we should use for updates.
					stable - Stable releases.
					beta - Beta releases.
					nightly - Nightly releases (unstable).
					testing - Testing releases (do not use).
					current - Whatever track this executable was released on'",

			ARG arg_chain: String = "foundation", or |c: &Config| otry!(c.parity).chain.clone(),
			"--chain CHAIN
				'Specify the blockchain type. CHAIN may be either a JSON chain specification file or olympic, frontier, homestead, mainnet, morden, ropsten, classic, expanse, testnet, kovan or dev.'",

			ARG arg_keys_path: String = "$BASE/keys", or |c: &Config| otry!(c.parity).keys_path.clone(),
			"--keys-path PATH
				'Specify the path for JSON key files to be found'",

			ARG arg_identity: String = "", or |c: &Config| otry!(c.parity).identity.clone(),
			"--identity NAME
				'Specify your node's name.'",

		["Convenience options"]
			FLAG flag_unsafe_expose: bool = false, or |c: &Config| otry!(c.misc).unsafe_expose,
			"--unsafe-expose
				'All servers will listen on external interfaces and will be remotely accessible. It's equivalent with setting the following: --{{ws,jsonrpc,ui,ipfs,secret_store,stratum}}-interface=all --*-hosts=all{n}This option is UNSAFE and should be used with great care!'",

			ARG arg_config: String = "$BASE/config.toml", or |_| None,
			"-c --config CONFIG
				'Specify a filename containing a configuration file.'",

			ARG arg_ports_shift: u16 = 0u16, or |c: &Config| otry!(c.misc).ports_shift,
			"--ports-shift SHIFT
				'Add SHIFT to all port numbers Parity is listening on. Includes network port and all servers (RPC, WebSockets, UI, IPFS, SecretStore).'",

		["Account options"]
			FLAG flag_no_hardware_wallets: bool = false, or |c: &Config| otry!(c.account).disable_hardware.clone(),
			"--no-hardware-wallets
				'Disables hardware wallet support.'",

			FLAG flag_fast_unlock: bool = false, or |c: &Config| otry!(c.account).fast_unlock.clone(),
			"--fast-unlock
				'Use drasticly faster unlocking mode. This setting causes raw secrets to be stored unprotected in memory, so use with care.'",

			ARG arg_unlock: Option<String> = None, or |c: &Config| otry!(c.account).unlock.as_ref().map(|vec| Some(vec.join(","))),
			"--unlock ACCOUNTS
				'Unlock ACCOUNTS for the duration of the execution. ACCOUNTS is a comma-delimited list of addresses. Implies --no-ui.'",

			ARG arg_password: Vec<String> = Vec::new(), or |c: &Config| otry!(c.account).password.clone(),
			"--password FILE
				'Provide a file containing a password for unlocking an account. Leading and trailing whitespace is trimmed.'",
			
			ARG arg_keys_iterations: u32 = 10240u32, or |c: &Config| otry!(c.account).keys_iterations.clone(),
			"--keys-iterations NUM
				'Specify the number of iterations to use when deriving key from the password (bigger is more secure)'",

		["UI options"]
			FLAG flag_force_ui: bool = false, or |c: &Config| otry!(c.ui).force.clone(),
			"--force-ui
				'Enable Trusted UI WebSocket endpoint, even when --unlock is in use.'",
			
			FLAG flag_no_ui: bool = false, or |c: &Config| otry!(c.ui).disable.clone(),
			"--no-ui
				'Disable Trusted UI WebSocket endpoint.'",

			// NOTE [todr] For security reasons don't put this to config files
			FLAG flag_ui_no_validation: bool = false, or |_| None,
			"--ui-no-validation
				'Disable Origin and Host headers validation for Trusted UI. WARNING: INSECURE. Used only for development.'",

			ARG arg_ui_interface: String = "local", or |c: &Config| otry!(c.ui).interface.clone(),
			"--ui-interface IP
				'Specify the hostname portion of the Trusted UI server, IP should be an interface's IP address, or local.'",

			ARG arg_ui_hosts: String = "none", or |c: &Config| otry!(c.ui).hosts.as_ref().map(|vec| vec.join(",")),
			"--ui-hosts HOSTS
				'List of allowed Host header values. This option will validate the Host header sent by the browser, it is additional security against some attack vectors. Special options: \"all\", \"none\",.'",

			ARG arg_ui_path: String = "$BASE/signer", or |c: &Config| otry!(c.ui).path.clone(),
			"--ui-path PATH
				'Specify directory where Trusted UIs tokens should be stored.'",

			ARG arg_ui_port: u16 = 8180u16, or |c: &Config| otry!(c.ui).port.clone(),
			"--ui-port PORT
				'Specify the port of Trusted UI server.'",

		["Networking options"]
			FLAG flag_no_warp: bool = false, or |c: &Config| otry!(c.network).warp.clone().map(|w| !w),
			"--no-warp
				'Disable syncing from the snapshot over the network.'",
			
			FLAG flag_no_discovery: bool = false, or |c: &Config| otry!(c.network).discovery.map(|d| !d).clone(),
			"--no-discovery
				'Disable new peer discovery.'",

			FLAG flag_reserved_only: bool = false, or |c: &Config| otry!(c.network).reserved_only.clone(),
			"--reserved-only
				'Connect only to reserved nodes.'",

			FLAG flag_no_ancient_blocks: bool = false, or |_| None,
			"--no-ancient-blocks
				'Disable downloading old blocks after snapshot restoration or warp sync.'",

			FLAG flag_no_serve_light: bool = false, or |c: &Config| otry!(c.network).no_serve_light.clone(),
			"--no-serve-light
				'Disable serving of light peers.'",

			ARG arg_port: u16 = 30303u16, or |c: &Config| otry!(c.network).port.clone(),
			"--port PORT
				'Override the port on which the node should listen.'",

			ARG arg_min_peers: u16 = 25u16, or |c: &Config| otry!(c.network).min_peers.clone(),
			"--min-peers NUM
				'Try to maintain at least NUM peers.'",

			ARG arg_max_peers: u16 = 50u16, or |c: &Config| otry!(c.network).max_peers.clone(),
			"--max-peers NUM
				'Allow up to NUM peers.'",

			ARG arg_snapshot_peers: u16 = 0u16, or |c: &Config| otry!(c.network).snapshot_peers.clone(),
			"--snapshot-peers NUM
				'Allow additional NUM peers for a snapshot sync.'",

			ARG arg_nat: String = "any", or |c: &Config| otry!(c.network).nat.clone(),
			"--nat METHOD
				'Specify method to use for determining public address. Must be one of: any, none, upnp, extip:<IP>.'",

			ARG arg_network_id: Option<u64> = None, or |c: &Config| otry!(c.network).id.clone().map(Some),
			"--network-id INDEX
				'Override the network identifier from the chain we are on.'",

			ARG arg_bootnodes: Option<String> = None, or |c: &Config| otry!(c.network).bootnodes.as_ref().map(|vec| Some(vec.join(","))),
			"--bootnodes NODES
				'Override the bootnodes from our chain. NODES should be comma-delimited enodes.'",

			ARG arg_node_key: Option<String> = None, or |c: &Config| otry!(c.network).node_key.clone().map(Some),
			"--node-key KEY
				'Specify node secret key, either as 64-character hex string or input to SHA3 operation.'",

			ARG arg_reserved_peers: Option<String> = None, or |c: &Config| otry!(c.network).reserved_peers.clone().map(Some),
			"--reserved-peers FILE
				'Provide a file containing enodes, one per line. These nodes will always have a reserved slot on top of the normal maximum peers.'",

			ARG arg_allow_ips: String = "all", or |c: &Config| otry!(c.network).allow_ips.clone(),
			"--allow-ips FILTER
				'Filter outbound connections. Must be one of: private - connect to private network IP addresses only; public - connect to public network IP addresses only; all - connect to any IP address.'",

			ARG arg_max_pending_peers: u16 = 64u16, or |c: &Config| otry!(c.network).max_pending_peers.clone(),
			"--max-pending-peers NUM
				'Allow up to NUM pending connections.'",

		["API and Console options: RPC"]
			// RPC
			FLAG flag_no_jsonrpc: bool = false, or |c: &Config| otry!(c.rpc).disable.clone(),
			"--no-jsonrpc
				'Disable the JSON-RPC API server.'", // API and Console Options:

			ARG arg_jsonrpc_port: u16 = 8545u16, or |c: &Config| otry!(c.rpc).port.clone(),
			"--jsonrpc-port PORT
				'Specify the port portion of the JSONRPC API server.'",

			ARG arg_jsonrpc_interface: String  = "local", or |c: &Config| otry!(c.rpc).interface.clone(),
			"--jsonrpc-interface IP
				'Specify the hostname portion of the JSONRPC API server, IP should be an interface's IP address, or all (all interfaces) or local.'",

			ARG arg_jsonrpc_cors: Option<String> = None, or |c: &Config| otry!(c.rpc).cors.clone().map(Some),
			"--jsonrpc-cors URL
				'Specify CORS header for JSON-RPC API responses.'",

			ARG arg_jsonrpc_apis: String = "web3,eth,pubsub,net,parity,parity_pubsub,traces,rpc,secretstore", or |c: &Config| otry!(c.rpc).apis.as_ref().map(|vec| vec.join(",")),
			"--jsonrpc-apis APIS
				'Specify the APIs available through the JSONRPC interface. APIS is a comma-delimited list of API name. Possible name are all, safe, web3, eth, net, personal, parity, parity_set, traces, rpc, parity_accounts. You can also disable a specific API by putting '-' in the front: all,-personal.'",

			ARG arg_jsonrpc_hosts: String = "none", or |c: &Config| otry!(c.rpc).hosts.as_ref().map(|vec| vec.join(",")),
			"--jsonrpc-hosts HOSTS
				'List of allowed Host header values. This option will validate the Host header sent by the browser, it is additional security against some attack vectors. Special options: \"all\", \"none\",.'",

			ARG arg_jsonrpc_server_threads: Option<usize> = None, or |c: &Config| otry!(c.rpc).server_threads.map(Some),
			"--jsonrpc-server-threads NUM
				'Enables experimental faster implementation of JSON-RPC server. Requires Dapps server to be disabled using --no-dapps.'",

			ARG arg_jsonrpc_threads: usize = 0usize, or |c: &Config| otry!(c.rpc).processing_threads,
			"--jsonrpc-threads THREADS
				'Turn on additional processing threads in all RPC servers. Setting this to non-zero value allows parallel cpu-heavy queries execution.'",

		["API and Console options: WS"]
			FLAG flag_no_ws: bool = false, or |c: &Config| otry!(c.websockets).disable.clone(),
			"--no-ws
				'Disable the WebSockets server.'",

			ARG arg_ws_port: u16 = 8546u16, or |c: &Config| otry!(c.websockets).port.clone(),
			"--ws-port PORT
				'Specify the port portion of the WebSockets server.'",

			ARG arg_ws_interface: String  = "local", or |c: &Config| otry!(c.websockets).interface.clone(),
			"--ws-interface IP
				'Specify the hostname portion of the WebSockets server, IP should be an interface's IP address, or all (all interfaces) or local.'",

			ARG arg_ws_apis: String = "web3,eth,pubsub,net,parity,parity_pubsub,traces,rpc,secretstore", or |c: &Config| otry!(c.websockets).apis.as_ref().map(|vec| vec.join(",")),
			"--ws-apis APIS
				'Specify the APIs available through the WebSockets interface. APIS is a comma-delimited list of API name. Possible name are web3, eth, pubsub, net, personal, parity, parity_set, traces, rpc, parity_accounts..'",

			ARG arg_ws_origins: String = "chrome-extension://*", or |c: &Config| otry!(c.websockets).origins.as_ref().map(|vec| vec.join(",")),
			"--ws-origins URL
				'Specify Origin header values allowed to connect. Special options: \"all\", \"none\".'",

			ARG arg_ws_hosts: String = "none", or |c: &Config| otry!(c.websockets).hosts.as_ref().map(|vec| vec.join(",")),
			"--ws-hosts HOSTS
				'List of allowed Host header values. This option will validate the Host header sent by the browser, it is additional security against some attack vectors. Special options: \"all\", \"none\",.'",

		["API and Console options: IPC"]
			FLAG flag_no_ipc: bool = false, or |c: &Config| otry!(c.ipc).disable.clone(),
			"--no-ipc
				'Disable JSON-RPC over IPC service.'",

			ARG arg_ipc_path: String = if cfg!(windows) { r"\\.\pipe\jsonrpc.ipc" } else { "$BASE/jsonrpc.ipc" }, or |c: &Config| otry!(c.ipc).path.clone(),
			"--ipc-path PATH
				'Specify custom path for JSON-RPC over IPC service.'",

			ARG arg_ipc_apis: String = "web3,eth,pubsub,net,parity,parity_pubsub,parity_accounts,traces,rpc,secretstore", or |c: &Config| otry!(c.ipc).apis.as_ref().map(|vec| vec.join(",")),
			"--ipc-apis APIS
				'Specify custom API set available via JSON-RPC over IPC.'",

		["API and Console options: Dapps"]
			FLAG flag_no_dapps: bool = false, or |c: &Config| otry!(c.dapps).disable.clone(),
			"--no-dapps
				'Disable the Dapps server (e.g. status page).'",

			ARG arg_dapps_path: String = "$BASE/dapps", or |c: &Config| otry!(c.dapps).path.clone(),
			"--dapps-path PATH
				'Specify directory where dapps should be installed.'",

		["API and Console options: IPFS"]
			FLAG flag_ipfs_api: bool = false, or |c: &Config| otry!(c.ipfs).enable.clone(),
			"--ipfs-api
				'Enable IPFS-compatible HTTP API.'",

			ARG arg_ipfs_api_port: u16 = 5001u16, or |c: &Config| otry!(c.ipfs).port.clone(),
			"--ipfs-api-port PORT
				'Configure on which port the IPFS HTTP API should listen.'",

			ARG arg_ipfs_api_interface: String = "local", or |c: &Config| otry!(c.ipfs).interface.clone(),
			"--ipfs-api-interface IP
				'Specify the hostname portion of the IPFS API server, IP should be an interface's IP address or local.'",

			ARG arg_ipfs_api_cors: Option<String> = None, or |c: &Config| otry!(c.ipfs).cors.clone().map(Some),
			"--ipfs-api-cors URL
				'Specify CORS header for IPFS API responses.'",

			ARG arg_ipfs_api_hosts: String = "none", or |c: &Config| otry!(c.ipfs).hosts.as_ref().map(|vec| vec.join(",")),
			"--ipfs-api-hosts HOSTS
				'List of allowed Host header values. This option will validate the Host header sent by the browser, it is additional security against some attack vectors. Special options: \"all\", \"none\".'",

		["Secret store options"]
			FLAG flag_no_secretstore: bool = false, or |c: &Config| otry!(c.secretstore).disable.clone(),
			"--no-secretstore
				'Disable Secret Store functionality.'",

			ARG arg_secretstore_secret: Option<String> = None, or |c: &Config| otry!(c.secretstore).self_secret.clone().map(Some),
			"--secretstore-secret SECRET
				'Hex-encoded secret key of this node. (required, default: {arg_secretstore_secret:?}).'",

			ARG arg_secretstore_nodes: String = "", or |c: &Config| otry!(c.secretstore).nodes.as_ref().map(|vec| vec.join(",")),
			"--secretstore-nodes NODES
				'Comma-separated list of other secret store cluster nodes in form NODE_PUBLIC_KEY_IN_HEX@NODE_IP_ADDR:NODE_PORT. (required, default: {arg_secretstore_nodes}).'",

			ARG arg_secretstore_interface: String = "local", or |c: &Config| otry!(c.secretstore).interface.clone(),
			"--secretstore-interface IP
				'Specify the hostname portion for listening to Secret Store Key Server internal requests, IP should be an interface's IP address, or local.'",

			ARG arg_secretstore_port: u16 = 8083u16, or |c: &Config| otry!(c.secretstore).port.clone(),
			"--secretstore-port PORT
				'Specify the port portion for listening to Secret Store Key Server internal requests.'",

			ARG arg_secretstore_http_interface: String = "local", or |c: &Config| otry!(c.secretstore).http_interface.clone(),
			"--secretstore-http-interface IP
				'Specify the hostname portion for listening to Secret Store Key Server HTTP requests, IP should be an interface's IP address, or local.'",

			ARG arg_secretstore_http_port: u16 = 8082u16, or |c: &Config| otry!(c.secretstore).http_port.clone(),
			"--secretstore-http-port PORT
				'Specify the port portion for listening to Secret Store Key Server HTTP requests.'",

			ARG arg_secretstore_path: String = "$BASE/secretstore", or |c: &Config| otry!(c.secretstore).path.clone(),
			"--secretstore-path PATH
				'Specify directory where Secret Store should save its data..'",

		["Sealing/Mining options"]
			FLAG flag_force_sealing: bool = false, or |c: &Config| otry!(c.mining).force_sealing.clone(),
			"--force-sealing
				'Force the node to author new blocks as if it were always sealing/mining.'",

			FLAG flag_reseal_on_uncle: bool = false, or |c: &Config| otry!(c.mining).reseal_on_uncle.clone(),
			"--reseal-on-uncle
				'Force the node to author new blocks when a new uncle block is imported.'",

			FLAG flag_remove_solved: bool = false, or |c: &Config| otry!(c.mining).remove_solved.clone(),
			"--remove-solved
				'Move solved blocks from the work package queue instead of cloning them. This gives a slightly faster import speed, but means that extra solutions submitted for the same work package will go unused.'",

			FLAG flag_refuse_service_transactions: bool = false, or |c: &Config| otry!(c.mining).refuse_service_transactions.clone(),
			"--refuse-service-transactions
				'Always refuse service transactions..'",

			FLAG flag_no_persistent_txqueue: bool = false, or |c: &Config| otry!(c.parity).no_persistent_txqueue,
			"--no-persistent-txqueue
				'Don't save pending local transactions to disk to be restored whenever the node restarts.'",

			FLAG flag_stratum: bool = false, or |c: &Config| Some(c.stratum.is_some()),
			"--stratum
				'Run Stratum server for miner push notification.'",

			ARG arg_author: Option<String> = None, or |c: &Config| otry!(c.mining).author.clone().map(Some),
			"--author ADDRESS
				'Specify the block author (aka \"coinbase\") address for sending block rewards from sealed blocks. NOTE: MINING WILL NOT WORK WITHOUT THIS OPTION.'", // Sealing/Mining Option

			ARG arg_engine_signer: Option<String> = None, or |c: &Config| otry!(c.mining).engine_signer.clone().map(Some),
			"--engine-signer ADDRESS
				'Specify the address which should be used to sign consensus messages and issue blocks. Relevant only to non-PoW chains.'",

			ARG arg_reseal_on_txs: String = "own", or |c: &Config| otry!(c.mining).reseal_on_txs.clone(),
			"--reseal-on-txs SET
				'Specify which transactions should force the node to reseal a block. SET is one of: none - never reseal on new transactions; own - reseal only on a new local transaction; ext - reseal only on a new external transaction; all - reseal on all new transactions.'",
			
			ARG arg_reseal_min_period: u64 = 2000u64, or |c: &Config| otry!(c.mining).reseal_min_period.clone(),
			"--reseal-min-period MS
				'Specify the minimum time between reseals from incoming transactions. MS is time measured in milliseconds.'",

			ARG arg_reseal_max_period: u64 = 120000u64, or |c: &Config| otry!(c.mining).reseal_max_period.clone(),
			"--reseal-max-period MS
				'Specify the maximum time since last block to enable force-sealing. MS is time measured in milliseconds.'",

			ARG arg_work_queue_size: usize = 20usize, or |c: &Config| otry!(c.mining).work_queue_size.clone(),
			"--work-queue-size ITEMS
				'Specify the number of historical work packages which are kept cached lest a solution is found for them later. High values take more memory but result in fewer unusable solutions.'",

			ARG arg_tx_gas_limit: Option<String> = None, or |c: &Config| otry!(c.mining).tx_gas_limit.clone().map(Some),
			"--tx-gas-limit GAS
				'Apply a limit of GAS as the maximum amount of gas a single transaction may have for it to be mined.'",

			ARG arg_tx_time_limit: Option<u64> = None, or |c: &Config| otry!(c.mining).tx_time_limit.clone().map(Some),
			"--tx-time-limit MS
				'Maximal time for processing single transaction. If enabled senders/recipients/code of transactions offending the limit will be banned from being included in transaction queue for 180 seconds.'",

			ARG arg_relay_set: String = "cheap", or |c: &Config| otry!(c.mining).relay_set.clone(),
			"--relay-set SET
				'Set of transactions to relay. SET may be: cheap - Relay any transaction in the queue (this may include invalid transactions); strict - Relay only executed transactions (this guarantees we don't relay invalid transactions, but means we relay nothing if not mining); lenient - Same as strict when mining, and cheap when not.'",

			ARG arg_usd_per_tx: String = "0.0025", or |c: &Config| otry!(c.mining).usd_per_tx.clone(),
			"--usd-per-tx USD
				'Amount of USD to be paid for a basic transaction. The minimum gas price is set accordingly.'",

			ARG arg_usd_per_eth: String = "auto", or |c: &Config| otry!(c.mining).usd_per_eth.clone(),
			"--usd-per-eth SOURCE
				'USD value of a single ETH. SOURCE may be either an amount in USD, a web service or 'auto' to use each web service in turn and fallback on the last known good value.'",

			ARG arg_price_update_period: String = "hourly", or |c: &Config| otry!(c.mining).price_update_period.clone(),
			"--price-update-period T
				'T will be allowed to pass between each gas price update. T may be daily, hourly, a number of seconds, or a time string of the form \"2 days\", \"30 minutes\" etc..'",

			ARG arg_gas_floor_target: String = "4700000", or |c: &Config| otry!(c.mining).gas_floor_target.clone(),
			"--gas-floor-target GAS
				'Amount of gas per block to target when sealing a new block.'",

			ARG arg_gas_cap: String = "6283184", or |c: &Config| otry!(c.mining).gas_cap.clone(),
			"--gas-cap GAS
				'A cap on how large we will raise the gas limit per block due to transaction volume.'",

			ARG arg_extra_data: Option<String> = None, or |c: &Config| otry!(c.mining).extra_data.clone().map(Some),
			"--extra-data STRING
				'Specify a custom extra-data for authored blocks, no more than 32 characters.'",

			ARG arg_tx_queue_mem_limit: u32 = 2u32, or |c: &Config| otry!(c.mining).tx_queue_mem_limit.clone(),
			"--tx-queue-mem-limit MB
				'Maximum amount of memory that can be used by the transaction queue. Setting this parameter to 0 disables limiting.'",

			ARG arg_tx_queue_size: usize = 8192usize, or |c: &Config| otry!(c.mining).tx_queue_size.clone(),
			"--tx-queue-size LIMIT
				'Maximum amount of transactions in the queue (waiting to be included in next block).'",

			ARG arg_tx_queue_gas: String = "off", or |c: &Config| otry!(c.mining).tx_queue_gas.clone(),
			"--tx-queue-gas LIMIT
				'Maximum amount of total gas for external transactions in the queue. LIMIT can be either an amount of gas or 'auto' or 'off'. 'auto' sets the limit to be 20x the current block gas limit..'",

			ARG arg_tx_queue_strategy: String = "gas_price", or |c: &Config| otry!(c.mining).tx_queue_strategy.clone(),
			"--tx-queue-strategy S
				'Prioritization strategy used to order transactions in the queue. S may be: gas - Prioritize txs with low gas limit; gas_price - Prioritize txs with high gas price; gas_factor - Prioritize txs using gas price and gas limit ratio.'",

			ARG arg_tx_queue_ban_count: u16 = 1u16, or |c: &Config| otry!(c.mining).tx_queue_ban_count.clone(),
			"--tx-queue-ban-count C
				'Number of times maximal time for execution (--tx-time-limit) can be exceeded before banning sender/recipient/code.'",

			ARG arg_tx_queue_ban_time: u16 = 180u16, or |c: &Config| otry!(c.mining).tx_queue_ban_time.clone(),
			"--tx-queue-ban-time SEC
				'Banning time (in seconds) for offenders of specified execution time limit. Also number of offending actions have to reach the threshold within that time. (default: {arg_tx_queue_ban_time} seconds)'",

			ARG arg_notify_work: Option<String> = None, or |c: &Config| otry!(c.mining).notify_work.as_ref().map(|vec| Some(vec.join(","))),
			"--notify-work URLS
				'URLs to which work package notifications are pushed. URLS should be a comma-delimited list of HTTP URLs.'",

			ARG arg_stratum_interface: String = "local", or |c: &Config| otry!(c.stratum).interface.clone(),
			"--stratum-interface IP
				'Interface address for Stratum server.'",

			ARG arg_stratum_port: u16 = 8008u16, or |c: &Config| otry!(c.stratum).port.clone(),
			"--stratum-port PORT
				'Port for Stratum server to listen on.'",

			ARG arg_stratum_secret: Option<String> = None, or |c: &Config| otry!(c.stratum).secret.clone().map(Some),
			"--stratum-secret STRING
				'Secret for authorizing Stratum server for peers.'",

		["Miscellaneous options"]
			FLAG flag_no_color: bool = false, or |c: &Config| otry!(c.misc).color.map(|c| !c).clone(),
			"--no-color
				'Don't use terminal color codes in output.'",

			ARG arg_ntp_server: String = "pool.ntp.org:123", or |c: &Config| otry!(c.misc).ntp_server.clone(),
			"--ntp-server HOST
				'NTP server to provide current time (host:port). Used to verify node health.'",

			ARG arg_logging: Option<String> = None, or |c: &Config| otry!(c.misc).logging.clone().map(Some),
			"-l --logging LOGGING
				'Specify the logging level. Must conform to the same format as RUST_LOG.'",

			ARG arg_log_file: Option<String> = None, or |c: &Config| otry!(c.misc).log_file.clone().map(Some),
			"--log-file FILENAME
				'Specify a filename into which logging should be appended.'",

		["Footprint options"]
			FLAG flag_fast_and_loose: bool = false, or |c: &Config| otry!(c.footprint).fast_and_loose.clone(),
			"--fast-and-loose
				'Disables DB WAL, which gives a significant speed up but means an unclean exit is unrecoverable.'",

			FLAG flag_scale_verifiers: bool = false, or |c: &Config| otry!(c.footprint).scale_verifiers.clone(),
			"--scale-verifiers
				'Automatically scale amount of verifier threads based on workload. Not guaranteed to be faster.'",

			ARG arg_tracing: String = "auto", or |c: &Config| otry!(c.footprint).tracing.clone(),
			"--tracing BOOL
				'Indicates if full transaction tracing should be enabled. Works only if client had been fully synced with tracing enabled. BOOL may be one of auto, on, off. auto uses last used value of this option (off if it does not exist).'", // footprint option

			ARG arg_pruning: String = "auto", or |c: &Config| otry!(c.footprint).pruning.clone(),
			"--pruning METHOD
				'Configure pruning of the state/storage trie. METHOD may be one of auto, archive, fast: archive - keep all state trie data. No pruning. fast - maintain journal overlay. Fast but 50MB used. auto - use the method most recently synced or default to fast if none synced.'",

			ARG arg_pruning_history: u64 = 64u64, or |c: &Config| otry!(c.footprint).pruning_history.clone(),
			"--pruning-history NUM
				'Set a minimum number of recent states to keep when pruning is active..'",

			ARG arg_pruning_memory: usize = 32usize, or |c: &Config| otry!(c.footprint).pruning_memory.clone(),
			"--pruning-memory MB
				'The ideal amount of memory in megabytes to use to store recent states. As many states as possible will be kept within this limit, and at least --pruning-history states will always be kept.'",

			ARG arg_cache_size_db: u32 = 32u32, or |c: &Config| otry!(c.footprint).cache_size_db.clone(),
			"--cache-size-db MB
				'Override database cache size.'",

			ARG arg_cache_size_blocks: u32 = 8u32, or |c: &Config| otry!(c.footprint).cache_size_blocks.clone(),
			"--cache-size-blocks MB
				'Specify the prefered size of the blockchain cache in megabytes.'",

			ARG arg_cache_size_queue: u32 = 40u32, or |c: &Config| otry!(c.footprint).cache_size_queue.clone(),
			"--cache-size-queue MB
				'Specify the maximum size of memory to use for block queue.'",

			ARG arg_cache_size_state: u32 = 25u32, or |c: &Config| otry!(c.footprint).cache_size_state.clone(),
			"--cache-size-state MB
				'Specify the maximum size of memory to use for the state cache.'",

			ARG arg_cache_size: Option<u32> = None, or |c: &Config| otry!(c.footprint).cache_size.clone().map(Some),
			"--cache-size MB
				'Set total amount of discretionary memory to use for the entire system, overrides other cache and queue options.'",

			ARG arg_db_compaction: String = "auto", or |c: &Config| otry!(c.footprint).db_compaction.clone(),
			"--db-compaction TYPE
				'Database compaction type. TYPE may be one of: ssd - suitable for SSDs and fast HDDs; hdd - suitable for slow HDDs; auto - determine automatically.'",

			ARG arg_fat_db: String = "auto", or |c: &Config| otry!(c.footprint).fat_db.clone(),
			"--fat-db BOOL
				'Build appropriate information to allow enumeration of all accounts and storage keys. Doubles the size of the state database. BOOL may be one of on, off or auto.'",

			ARG arg_num_verifiers: Option<usize> = None, or |c: &Config| otry!(c.footprint).num_verifiers.clone().map(Some),
			"--num-verifiers INT
				'Amount of verifier threads to use or to begin with, if verifier auto-scaling is enabled.'",

		["Import/export options"]
			FLAG flag_no_seal_check: bool = false, or |_| None,
			"--no-seal-check
				'Skip block seal check.'",

			FLAG flag_no_storage: bool = false, or |_| None,
			"--no-storage
				'Don't export account storage.'",

			FLAG flag_no_code: bool = false, or |_| None,
			"--no-code
				'Don't export account code.'",

			ARG arg_from: String = "1", or |_| None,
			"--from BLOCK
				'Export from block BLOCK, which may be an index or hash.'", // Import/Export Options:

			ARG arg_to: String = "latest", or |_| None,
			"--to BLOCK
				'Export to (including) block BLOCK, which may be an index, hash or 'latest'.'",

			ARG arg_format: Option<String> = None, or |_| None,
			"--format FORMAT
				'For import/export in given format. FORMAT must be one of 'hex' and 'binary'. (default: {arg_format:?} = Import: auto, Export: binary)'",

			ARG arg_min_balance: Option<String> = None, or |_| None,
			"--min-balance WEI
				'Don't export accounts with balance less than specified.'",

			ARG arg_max_balance: Option<String> = None, or |_| None,
			"--max-balance WEI
				'Don't export accounts with balance greater than specified.'",

		["Snapshot options"]
			FLAG flag_no_periodic_snapshot: bool = false, or |c: &Config| otry!(c.snapshots).disable_periodic.clone(),
			"--no-periodic-snapshot
				'Disable automated snapshots which usually occur once every 10000 blocks.'",

			ARG arg_at: String = "latest", or |_| None,
			"--at BLOCK
				'Take a snapshot at the given block, which may be an index, hash, or 'latest'. Note that taking snapshots at non-recent blocks will only work with --pruning archive'", // Snapshot Option

		["Virtual Machine Options"]
			FLAG flag_jitvm: bool = false, or |c: &Config| otry!(c.vm).jit.clone(),
			"--jitvm
				'Enable the JIT VM.'",
	}
}

#[derive(Default, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
	parity: Option<Operating>,
	account: Option<Account>,
	ui: Option<Ui>,
	network: Option<Network>,
	rpc: Option<Rpc>,
	websockets: Option<Ws>,
	ipc: Option<Ipc>,
	dapps: Option<Dapps>,
	secretstore: Option<SecretStore>,
	ipfs: Option<Ipfs>,
	mining: Option<Mining>,
	footprint: Option<Footprint>,
	snapshots: Option<Snapshots>,
	vm: Option<VM>,
	misc: Option<Misc>,
	stratum: Option<Stratum>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Operating {
	mode: Option<String>,
	mode_timeout: Option<u64>,
	mode_alarm: Option<u64>,
	auto_update: Option<String>,
	release_track: Option<String>,
	public_node: Option<bool>,
	no_download: Option<bool>,
	no_consensus: Option<bool>,
	chain: Option<String>,
	base_path: Option<String>,
	db_path: Option<String>,
	keys_path: Option<String>,
	identity: Option<String>,
	light: Option<bool>,
	no_persistent_txqueue: Option<bool>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Account {
	unlock: Option<Vec<String>>,
	password: Option<Vec<String>>,
	keys_iterations: Option<u32>,
	disable_hardware: Option<bool>,
	fast_unlock: Option<bool>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Ui {
	force: Option<bool>,
	disable: Option<bool>,
	port: Option<u16>,
	interface: Option<String>,
	hosts: Option<Vec<String>>,
	path: Option<String>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Network {
	warp: Option<bool>,
	port: Option<u16>,
	min_peers: Option<u16>,
	max_peers: Option<u16>,
	snapshot_peers: Option<u16>,
	max_pending_peers: Option<u16>,
	nat: Option<String>,
	allow_ips: Option<String>,
	id: Option<u64>,
	bootnodes: Option<Vec<String>>,
	discovery: Option<bool>,
	node_key: Option<String>,
	reserved_peers: Option<String>,
	reserved_only: Option<bool>,
	no_serve_light: Option<bool>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Rpc {
	disable: Option<bool>,
	port: Option<u16>,
	interface: Option<String>,
	cors: Option<String>,
	apis: Option<Vec<String>>,
	hosts: Option<Vec<String>>,
	server_threads: Option<usize>,
	processing_threads: Option<usize>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Ws {
	disable: Option<bool>,
	port: Option<u16>,
	interface: Option<String>,
	apis: Option<Vec<String>>,
	origins: Option<Vec<String>>,
	hosts: Option<Vec<String>>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Ipc {
	disable: Option<bool>,
	path: Option<String>,
	apis: Option<Vec<String>>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Dapps {
	disable: Option<bool>,
	port: Option<u16>,
	interface: Option<String>,
	hosts: Option<Vec<String>>,
	cors: Option<String>,
	path: Option<String>,
	user: Option<String>,
	pass: Option<String>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct SecretStore {
	disable: Option<bool>,
	self_secret: Option<String>,
	nodes: Option<Vec<String>>,
	interface: Option<String>,
	port: Option<u16>,
	http_interface: Option<String>,
	http_port: Option<u16>,
	path: Option<String>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Ipfs {
	enable: Option<bool>,
	port: Option<u16>,
	interface: Option<String>,
	cors: Option<String>,
	hosts: Option<Vec<String>>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Mining {
	author: Option<String>,
	engine_signer: Option<String>,
	force_sealing: Option<bool>,
	reseal_on_uncle: Option<bool>,
	reseal_on_txs: Option<String>,
	reseal_min_period: Option<u64>,
	reseal_max_period: Option<u64>,
	work_queue_size: Option<usize>,
	tx_gas_limit: Option<String>,
	tx_time_limit: Option<u64>,
	relay_set: Option<String>,
	usd_per_tx: Option<String>,
	usd_per_eth: Option<String>,
	price_update_period: Option<String>,
	gas_floor_target: Option<String>,
	gas_cap: Option<String>,
	extra_data: Option<String>,
	tx_queue_size: Option<usize>,
	tx_queue_mem_limit: Option<u32>,
	tx_queue_gas: Option<String>,
	tx_queue_strategy: Option<String>,
	tx_queue_ban_count: Option<u16>,
	tx_queue_ban_time: Option<u16>,
	remove_solved: Option<bool>,
	notify_work: Option<Vec<String>>,
	refuse_service_transactions: Option<bool>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Stratum {
	interface: Option<String>,
	port: Option<u16>,
	secret: Option<String>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Footprint {
	tracing: Option<String>,
	pruning: Option<String>,
	pruning_history: Option<u64>,
	pruning_memory: Option<usize>,
	fast_and_loose: Option<bool>,
	cache_size: Option<u32>,
	cache_size_db: Option<u32>,
	cache_size_blocks: Option<u32>,
	cache_size_queue: Option<u32>,
	cache_size_state: Option<u32>,
	db_compaction: Option<String>,
	fat_db: Option<String>,
	scale_verifiers: Option<bool>,
	num_verifiers: Option<usize>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Snapshots {
	disable_periodic: Option<bool>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct VM {
	jit: Option<bool>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Misc {
	ntp_server: Option<String>,
	logging: Option<String>,
	log_file: Option<String>,
	color: Option<bool>,
	ports_shift: Option<u16>,
	unsafe_expose: Option<bool>,
}

#[cfg(test)]
mod tests {
	use super::{
		Args, ArgsError,
		Config, Operating, Account, Ui, Network, Ws, Rpc, Ipc, Dapps, Ipfs, Mining, Footprint,
		Snapshots, VM, Misc, SecretStore,
	};
	use toml;

	#[test]
	fn should_parse_args_and_include_config() {
		// given
		let mut config = Config::default();
		let mut operating = Operating::default();
		operating.chain = Some("morden".into());
		config.parity = Some(operating);

		// when
		let args = Args::parse_with_config(&["parity"], config).unwrap();

		// then
		assert_eq!(args.arg_chain, "morden".to_owned());
	}

	#[test]
	fn should_not_use_config_if_cli_is_provided() {
		// given
		let mut config = Config::default();
		let mut operating = Operating::default();
		operating.chain = Some("morden".into());
		config.parity = Some(operating);

		// when
		let args = Args::parse_with_config(&["parity", "--chain", "xyz"], config).unwrap();

		// then
		assert_eq!(args.arg_chain, "xyz".to_owned());
	}

	#[test]
	fn should_use_config_if_cli_is_missing() {
		let mut config = Config::default();
		let mut footprint = Footprint::default();
		footprint.pruning_history = Some(128);
		config.footprint = Some(footprint);

		// when
		let args = Args::parse_with_config(&["parity"], config).unwrap();

		// then
		assert_eq!(args.arg_pruning_history, 128);
	}

	#[test]
	fn should_parse_full_config() {
		// given
		let config = toml::from_str(include_str!("./tests/config.full.toml")).unwrap();

		// when
		let args = Args::parse_with_config(&["parity", "--chain", "xyz"], config).unwrap();

		// then
		assert_eq!(args, Args {
			// Commands
			cmd_daemon: false,
			cmd_wallet: false,
			cmd_account: false,
			cmd_new: false,
			cmd_list: false,
			cmd_export: false,
			cmd_state: false,
			cmd_blocks: false,
			cmd_import: false,
			cmd_signer: false,
			cmd_sign: false,
			cmd_reject: false,
			cmd_new_token: false,
			cmd_snapshot: false,
			cmd_restore: false,
			cmd_ui: false,
			cmd_dapp: false,
			cmd_tools: false,
			cmd_hash: false,
			cmd_db: false,
			cmd_kill: false,

			// Arguments
			arg_pid_file: "".into(),
			arg_file: None,
			arg_id: None,
			arg_path: vec![],

			// -- Operating Options
			arg_mode: "last".into(),
			arg_mode_timeout: 300u64,
			arg_mode_alarm: 3600u64,
			arg_auto_update: "none".into(),
			arg_release_track: "current".into(),
			flag_public_node: false,
			flag_no_download: false,
			flag_no_consensus: false,
			arg_chain: "xyz".into(),
			flag_base_path: Some("$HOME/.parity".into()),
			flag_db_path: Some("$HOME/.parity/chains".into()),
			arg_keys_path: "$HOME/.parity/keys".into(),
			arg_identity: "".into(),
			flag_light: false,
			flag_no_persistent_txqueue: false,

			// -- Convenience Options
			arg_config: "$BASE/config.toml".into(),
			arg_ports_shift: 0,
			flag_unsafe_expose: false,

			// -- Account Options
			arg_unlock: Some("0xdeadbeefcafe0000000000000000000000000000".into()),
			arg_password: vec!["~/.safe/password.file".into()],
			arg_keys_iterations: 10240u32,
			flag_no_hardware_wallets: false,
			flag_fast_unlock: false,

			flag_force_ui: false,
			flag_no_ui: false,
			flag_ui_port: 8180u16,
			arg_ui_interface: "127.0.0.1".into(),
			arg_ui_hosts: "none".into(),
			arg_ui_path: "$HOME/.parity/signer".into(),
			flag_ui_no_validation: false,

			// -- Networking Options
			flag_no_warp: false,
			arg_port: 30303u16,
			arg_min_peers: 25u16,
			arg_max_peers: 50u16,
			arg_max_pending_peers: 64u16,
			arg_snapshot_peers: 0u16,
			arg_allow_ips: "all".into(),
			arg_nat: "any".into(),
			arg_network_id: Some(1),
			arg_bootnodes: Some("".into()),
			flag_no_discovery: false,
			arg_node_key: None,
			arg_reserved_peers: Some("./path_to_file".into()),
			flag_reserved_only: false,
			flag_no_ancient_blocks: false,
			flag_no_serve_light: false,

			// -- API and Console Options
			// RPC
			flag_no_jsonrpc: false,
			arg_jsonrpc_port: 8545u16,
			arg_jsonrpc_interface: "local".into(),
			arg_jsonrpc_cors: Some("null".into()),
			arg_jsonrpc_apis: "web3,eth,net,parity,traces,rpc,secretstore".into(),
			arg_jsonrpc_hosts: "none".into(),
			arg_jsonrpc_server_threads: None,
			arg_jsonrpc_threads: 0,

			// WS
			flag_no_ws: false,
			arg_ws_port: 8546u16,
			arg_ws_interface: "local".into(),
			arg_ws_apis: "web3,eth,net,parity,traces,rpc,secretstore".into(),
			arg_ws_origins: "none".into(),
			arg_ws_hosts: "none".into(),

			// IPC
			flag_no_ipc: false,
			arg_ipc_path: "$HOME/.parity/jsonrpc.ipc".into(),
			arg_ipc_apis: "web3,eth,net,parity,parity_accounts,personal,traces,rpc,secretstore".into(),

			// DAPPS
			arg_dapps_path: "$HOME/.parity/dapps".into(),
			flag_no_dapps: false,

			flag_no_secretstore: false,
			arg_secretstore_secret: None,
			arg_secretstore_nodes: "".into(),
			arg_secretstore_interface: "local".into(),
			arg_secretstore_port: 8083u16,
			arg_secretstore_http_interface: "local".into(),
			arg_secretstore_http_port: 8082u16,
			arg_secretstore_path: "$HOME/.parity/secretstore".into(),

			// IPFS
			flag_ipfs_api: false,
			arg_ipfs_api_port: 5001u16,
			arg_ipfs_api_interface: "local".into(),
			arg_ipfs_api_cors: Some("null".into()),
			arg_ipfs_api_hosts: "none".into(),

			// -- Sealing/Mining Options
			arg_author: Some("0xdeadbeefcafe0000000000000000000000000001".into()),
			arg_engine_signer: Some("0xdeadbeefcafe0000000000000000000000000001".into()),
			flag_force_sealing: true,
			arg_reseal_on_txs: "all".into(),
			arg_reseal_min_period: 4000u64,
			arg_reseal_max_period: 60000u64,
			flag_reseal_on_uncle: false,
			arg_work_queue_size: 20usize,
			arg_tx_gas_limit: Some("6283184".into()),
			arg_tx_time_limit: Some(100u64),
			arg_relay_set: "cheap".into(),
			arg_usd_per_tx: "0.0025".into(),
			arg_usd_per_eth: "auto".into(),
			arg_price_update_period: "hourly".into(),
			arg_gas_floor_target: "4700000".into(),
			arg_gas_cap: "6283184".into(),
			arg_extra_data: Some("Parity".into()),
			arg_tx_queue_size: 8192usize,
			arg_tx_queue_mem_limit: 2u32,
			arg_tx_queue_gas: "off".into(),
			arg_tx_queue_strategy: "gas_factor".into(),
			arg_tx_queue_ban_count: 1u16,
			arg_tx_queue_ban_time: 180u16,
			flag_remove_solved: false,
			arg_notify_work: Some("http://localhost:3001".into()),
			flag_refuse_service_transactions: false,

			flag_stratum: false,
			arg_stratum_interface: "local".to_owned(),
			arg_stratum_port: 8008u16,
			arg_stratum_secret: None,

			// -- Footprint Options
			arg_tracing: "auto".into(),
			arg_pruning: "auto".into(),
			arg_pruning_history: 64u64,
			arg_pruning_memory: 500usize,
			arg_cache_size_db: 64u32,
			arg_cache_size_blocks: 8u32,
			arg_cache_size_queue: 50u32,
			arg_cache_size_state: 25u32,
			arg_cache_size: Some(128),
			flag_fast_and_loose: false,
			arg_db_compaction: "ssd".into(),
			arg_fat_db: "auto".into(),
			flag_scale_verifiers: true,
			arg_num_verifiers: Some(6),

			// -- Import/Export Options
			arg_from: "1".into(),
			arg_to: "latest".into(),
			arg_format: None,
			flag_no_seal_check: false,
			flag_no_code: false,
			flag_no_storage: false,
			arg_min_balance: None,
			arg_max_balance: None,

			// -- Snapshot Optons
			arg_at: "latest".into(),
			flag_no_periodic_snapshot: false,

			// -- Virtual Machine Options
			flag_jitvm: false,

			// -- Legacy Options
			flag_geth: false,
			flag_testnet: false,
			flag_import_geth_keys: false,
			flag_datadir: None,
			flag_networkid: None,
			flag_peers: None,
			flag_nodekey: None,
			flag_nodiscover: false,
			flag_jsonrpc: false,
			flag_jsonrpc_off: false,
			flag_webapp: false,
			flag_dapps_off: false,
			flag_rpc: false,
			flag_rpcaddr: None,
			flag_rpcport: None,
			flag_rpcapi: None,
			flag_rpccorsdomain: None,
			flag_ipcdisable: false,
			flag_ipc_off: false,
			flag_ipcapi: None,
			flag_ipcpath: None,
			flag_gasprice: None,
			flag_etherbase: None,
			flag_extradata: None,
			flag_cache: None,
			flag_warp: Some(true),
			// Legacy-Dapps
			flag_dapps_port: Some(8080),
			flag_dapps_interface: Some("local".into()),
			flag_dapps_hosts: Some("none".into()),
			flag_dapps_cors: None,
			flag_dapps_user: Some("test_user".into()),
			flag_dapps_pass: Some("test_pass".into()),
			flag_dapps_apis_all: None,

			// -- Miscellaneous Options
			arg_ntp_server: "pool.ntp.org:123".into(),
			arg_logging: Some("own_tx=trace".into()),
			arg_log_file: Some("/var/log/parity.log".into()),
			flag_no_color: false,
			flag_no_config: false,
		});
	}

	#[test]
	fn should_parse_config_and_return_errors() {
		let config1 = Args::parse_config(include_str!("./tests/config.invalid1.toml"));
		let config2 = Args::parse_config(include_str!("./tests/config.invalid2.toml"));
		let config3 = Args::parse_config(include_str!("./tests/config.invalid3.toml"));

		match (config1, config2, config3) {
			(Err(ArgsError::Decode(_)), Err(ArgsError::Decode(_)), Err(ArgsError::Decode(_))) => {},
			(a, b, c) => {
				assert!(false, "Got invalid error types: {:?}, {:?}, {:?}", a, b, c);
			}
		}
	}

	#[test]
	fn should_deserialize_toml_file() {
		let config: Config = toml::from_str(include_str!("./tests/config.toml")).unwrap();

		assert_eq!(config, Config {
			parity: Some(Operating {
				mode: Some("dark".into()),
				mode_timeout: Some(15u64),
				mode_alarm: Some(10u64),
				auto_update: None,
				release_track: None,
				public_node: None,
				no_download: None,
				no_consensus: None,
				chain: Some("./chain.json".into()),
				base_path: None,
				db_path: None,
				keys_path: None,
				identity: None,
				light: None,
				no_persistent_txqueue: None,
			}),
			account: Some(Account {
				unlock: Some(vec!["0x1".into(), "0x2".into(), "0x3".into()]),
				password: Some(vec!["passwdfile path".into()]),
				keys_iterations: None,
				disable_hardware: None,
				fast_unlock: None,
			}),
			ui: Some(Ui {
				force: None,
				disable: Some(true),
				port: None,
				interface: None,
				hosts: None,
				path: None,
			}),
			network: Some(Network {
				warp: Some(false),
				port: None,
				min_peers: Some(10),
				max_peers: Some(20),
				max_pending_peers: Some(30),
				snapshot_peers: Some(40),
				allow_ips: Some("public".into()),
				nat: Some("any".into()),
				id: None,
				bootnodes: None,
				discovery: Some(true),
				node_key: None,
				reserved_peers: Some("./path/to/reserved_peers".into()),
				reserved_only: Some(true),
				no_serve_light: None,
			}),
			websockets: Some(Ws {
				disable: Some(true),
				port: None,
				interface: None,
				apis: None,
				origins: Some(vec!["none".into()]),
				hosts: None,
			}),
			rpc: Some(Rpc {
				disable: Some(true),
				port: Some(8180),
				interface: None,
				cors: None,
				apis: None,
				hosts: None,
				server_threads: None,
				processing_threads: None,
			}),
			ipc: Some(Ipc {
				disable: None,
				path: None,
				apis: Some(vec!["rpc".into(), "eth".into()]),
			}),
			dapps: Some(Dapps {
				disable: None,
				port: Some(8080),
				path: None,
				interface: None,
				hosts: None,
				cors: None,
				user: Some("username".into()),
				pass: Some("password".into())
			}),
			secretstore: Some(SecretStore {
				disable: None,
				self_secret: None,
				nodes: None,
				interface: None,
				port: Some(8083),
				http_interface: None,
				http_port: Some(8082),
				path: None,
			}),
			ipfs: Some(Ipfs {
				enable: Some(false),
				port: Some(5001),
				interface: None,
				cors: None,
				hosts: None,
			}),
			mining: Some(Mining {
				author: Some("0xdeadbeefcafe0000000000000000000000000001".into()),
				engine_signer: Some("0xdeadbeefcafe0000000000000000000000000001".into()),
				force_sealing: Some(true),
				reseal_on_txs: Some("all".into()),
				reseal_on_uncle: None,
				reseal_min_period: Some(4000),
				reseal_max_period: Some(60000),
				work_queue_size: None,
				relay_set: None,
				usd_per_tx: None,
				usd_per_eth: None,
				price_update_period: Some("hourly".into()),
				gas_floor_target: None,
				gas_cap: None,
				tx_queue_size: Some(8192),
				tx_queue_mem_limit: None,
				tx_queue_gas: Some("off".into()),
				tx_queue_strategy: None,
				tx_queue_ban_count: None,
				tx_queue_ban_time: None,
				tx_gas_limit: None,
				tx_time_limit: None,
				extra_data: None,
				remove_solved: None,
				notify_work: None,
				refuse_service_transactions: None,
			}),
			footprint: Some(Footprint {
				tracing: Some("on".into()),
				pruning: Some("fast".into()),
				pruning_history: Some(64),
				pruning_memory: None,
				fast_and_loose: None,
				cache_size: None,
				cache_size_db: Some(128),
				cache_size_blocks: Some(16),
				cache_size_queue: Some(100),
				cache_size_state: Some(25),
				db_compaction: Some("ssd".into()),
				fat_db: Some("off".into()),
				scale_verifiers: Some(false),
				num_verifiers: None,
			}),
			snapshots: Some(Snapshots {
				disable_periodic: Some(true),
			}),
			vm: Some(VM {
				jit: Some(false),
			}),
			misc: Some(Misc {
				ntp_server: Some("pool.ntp.org:123".into()),
				logging: Some("own_tx=trace".into()),
				log_file: Some("/var/log/parity.log".into()),
				color: Some(true),
				ports_shift: Some(0),
				unsafe_expose: Some(false),
			}),
			stratum: None,
		});
	}
}
