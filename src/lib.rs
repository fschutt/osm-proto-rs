extern crate prost;
#[macro_use]
extern crate prost_derive;

pub mod osmformat {
	// OSM Binary file format 
	//
	//This is the master schema file of the OSM binary file format. This
	//file is designed to support limited random-access and future
	//extendability.
	//
	//A binary OSM file consists of a sequence of FileBlocks (please see
	//fileformat.proto). The first fileblock contains a serialized instance
	//of HeaderBlock, followed by a sequence of PrimitiveBlock blocks that
	//contain the primitives.
	//
	//Each primitiveblock is designed to be independently parsable. It
	//contains a string table storing all strings in that block (keys and
	//values in tags, roles in relations, usernames, etc.) as well as
	//metadata containing the precision of coordinates or timestamps in that
	//block.
	//
	//A primitiveblock contains a sequence of primitive groups, each
	//containing primitives of the same type (nodes, densenodes, ways,
	//relations). Coordinates are stored in signed 64-bit integers. Lat&lon
	//are measured in units <granularity> nanodegrees. The default of
	//granularity of 100 nanodegrees corresponds to about 1cm on the ground,
	//and a full lat or lon fits into 32 bits.
	//
	//Converting an integer to a lattitude or longitude uses the formula:
	//$OUT = IN * granularity / 10**9$. Many encoding schemes use delta
	//coding when representing nodes and relations.
	//

	//////////////////////////////////////////////////////////////////////////
	//////////////////////////////////////////////////////////////////////////

	// Contains the file header. 

	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct HeaderBlock {
	    #[prost(message, optional, tag="1")]
	    pub bbox: Option<HeaderBBox>,
	    /// Additional tags to aid in parsing this dataset 
	    #[prost(string, repeated, tag="4")]
	    pub required_features: Vec<String>,
	    #[prost(string, repeated, tag="5")]
	    pub optional_features: Vec<String>,
	    #[prost(string, optional, tag="16")]
	    pub writingprogram: Option<String>,
	    /// From the bbox field.
	    #[prost(string, optional, tag="17")]
	    pub source: Option<String>,
	    // Tags that allow continuing an Osmosis replication 

	    /// replication timestamp, expressed in seconds since the epoch, 
	    /// otherwise the same value as in the "timestamp=..." field
	    /// in the state.txt file used by Osmosis
	    #[prost(int64, optional, tag="32")]
	    pub osmosis_replication_timestamp: Option<i64>,
	    /// replication sequence number (sequenceNumber in state.txt)
	    #[prost(int64, optional, tag="33")]
	    pub osmosis_replication_sequence_number: Option<i64>,
	    /// replication base URL (from Osmosis' configuration.txt file)
	    #[prost(string, optional, tag="34")]
	    pub osmosis_replication_base_url: Option<String>,
	}
	//* The bounding box field in the OSM header. BBOX, as used in the OSM
	//header. Units are always in nanodegrees -- they do not obey
	//granularity rules. 

	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct HeaderBBox {
	    #[prost(sint64, required, tag="1")]
	    pub left: i64,
	    #[prost(sint64, required, tag="2")]
	    pub right: i64,
	    #[prost(sint64, required, tag="3")]
	    pub top: i64,
	    #[prost(sint64, required, tag="4")]
	    pub bottom: i64,
	}
	///////////////////////////////////////////////////////////////////////
	///////////////////////////////////////////////////////////////////////

	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct PrimitiveBlock {
	    #[prost(message, required, tag="1")]
	    pub stringtable: StringTable,
	    #[prost(message, repeated, tag="2")]
	    pub primitivegroup: Vec<PrimitiveGroup>,
	    /// Granularity, units of nanodegrees, used to store coordinates in this block
	    #[prost(int32, optional, tag="17")]
	    pub granularity: Option<i32>,
	    /// Offset value between the output coordinates coordinates and the granularity grid in unites of nanodegrees.
	    #[prost(int64, optional, tag="19")]
	    pub lat_offset: Option<i64>,
	    #[prost(int64, optional, tag="20")]
	    pub lon_offset: Option<i64>,
	    /// Granularity of dates, normally represented in units of milliseconds since the 1970 epoch.
	    #[prost(int32, optional, tag="18")]
	    pub date_granularity: Option<i32>,
	}
	/// Group of OSMPrimitives. All primitives in a group must be the same type.
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct PrimitiveGroup {
	    #[prost(message, repeated, tag="1")]
	    pub nodes: Vec<Node>,
	    #[prost(message, optional, tag="2")]
	    pub dense: Option<DenseNodes>,
	    #[prost(message, repeated, tag="3")]
	    pub ways: Vec<Way>,
	    #[prost(message, repeated, tag="4")]
	    pub relations: Vec<Relation>,
	    #[prost(message, repeated, tag="5")]
	    pub changesets: Vec<ChangeSet>,
	}
	///* String table, contains the common strings in each block.
	///
	///Note that we reserve index '0' as a delimiter, so the entry at that
	///index in the table is ALWAYS blank and unused.
	///
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct StringTable {
	    #[prost(bytes, repeated, tag="1")]
	    pub s: Vec<Vec<u8>>,
	}
	/// Optional metadata that may be included into each primitive. 
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct Info {
	    #[prost(int32, optional, tag="1")]
	    pub version: Option<i32>,
	    #[prost(int64, optional, tag="2")]
	    pub timestamp: Option<i64>,
	    #[prost(int64, optional, tag="3")]
	    pub changeset: Option<i64>,
	    #[prost(int32, optional, tag="4")]
	    pub uid: Option<i32>,
	    /// String IDs
	    #[prost(uint32, optional, tag="5")]
	    pub user_sid: Option<u32>,
	    /// The visible flag is used to store history information. It indicates that
	    /// the current object version has been created by a delete operation on the
	    /// OSM API.
	    /// When a writer sets this flag, it MUST add a required_features tag with
	    /// value "HistoricalInformation" to the HeaderBlock.
	    /// If this flag is not available for some object it MUST be assumed to be
	    /// true if the file has the required_features tag "HistoricalInformation"
	    /// set.
	    #[prost(bool, optional, tag="6")]
	    pub visible: Option<bool>,
	}
	///* Optional metadata that may be included into each primitive. Special dense format used in DenseNodes. 
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct DenseInfo {
	    #[prost(int32, repeated, tag="1")]
	    pub version: Vec<i32>,
	    /// DELTA coded
	    #[prost(sint64, repeated, tag="2")]
	    pub timestamp: Vec<i64>,
	    /// DELTA coded
	    #[prost(sint64, repeated, tag="3")]
	    pub changeset: Vec<i64>,
	    /// DELTA coded
	    #[prost(sint32, repeated, tag="4")]
	    pub uid: Vec<i32>,
	    /// String IDs for usernames. DELTA coded
	    #[prost(sint32, repeated, tag="5")]
	    pub user_sid: Vec<i32>,
	    /// The visible flag is used to store history information. It indicates that
	    /// the current object version has been created by a delete operation on the
	    /// OSM API.
	    /// When a writer sets this flag, it MUST add a required_features tag with
	    /// value "HistoricalInformation" to the HeaderBlock.
	    /// If this flag is not available for some object it MUST be assumed to be
	    /// true if the file has the required_features tag "HistoricalInformation"
	    /// set.
	    #[prost(bool, repeated, tag="6")]
	    pub visible: Vec<bool>,
	}
	/// THIS IS STUB DESIGN FOR CHANGESETS. NOT USED RIGHT NOW.
	/// TODO:    REMOVE THIS?
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct ChangeSet {
	    ///   
	    ///   // Parallel arrays.
	    ///   repeated uint32 keys = 2 [packed = true]; // String IDs.
	    ///   repeated uint32 vals = 3 [packed = true]; // String IDs.
	    ///
	    ///   optional Info info = 4;
	    #[prost(int64, required, tag="1")]
	    pub id: i64,
	}
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct Node {
	    #[prost(sint64, required, tag="1")]
	    pub id: i64,
	    /// Parallel arrays.
	    /// String IDs.
	    #[prost(uint32, repeated, tag="2")]
	    pub keys: Vec<u32>,
	    /// String IDs.
	    #[prost(uint32, repeated, tag="3")]
	    pub vals: Vec<u32>,
	    /// May be omitted in omitmeta
	    #[prost(message, optional, tag="4")]
	    pub info: Option<Info>,
	    #[prost(sint64, required, tag="8")]
	    pub lat: i64,
	    #[prost(sint64, required, tag="9")]
	    pub lon: i64,
	}
	// Used to densly represent a sequence of nodes that do not have any tags.
	//
	//We represent these nodes columnwise as five columns: ID's, lats, and
	//lons, all delta coded. When metadata is not omitted, 
	//
	//We encode keys & vals for all nodes as a single array of integers
	//containing key-stringid and val-stringid, using a stringid of 0 as a
	//delimiter between nodes.
	//
	//( (<keyid> <valid>)* '0' )*

	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct DenseNodes {
	    /// DELTA coded
	    #[prost(sint64, repeated, tag="1")]
	    pub id: Vec<i64>,
	    ///repeated Info info = 4;
	    #[prost(message, optional, tag="5")]
	    pub denseinfo: Option<DenseInfo>,
	    /// DELTA coded
	    #[prost(sint64, repeated, tag="8")]
	    pub lat: Vec<i64>,
	    /// DELTA coded
	    #[prost(sint64, repeated, tag="9")]
	    pub lon: Vec<i64>,
	    /// Special packing of keys and vals into one array. May be empty if all nodes in this block are tagless.
	    #[prost(int32, repeated, tag="10")]
	    pub keys_vals: Vec<i32>,
	}
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct Way {
	    #[prost(int64, required, tag="1")]
	    pub id: i64,
	    /// Parallel arrays.
	    #[prost(uint32, repeated, tag="2")]
	    pub keys: Vec<u32>,
	    #[prost(uint32, repeated, tag="3")]
	    pub vals: Vec<u32>,
	    #[prost(message, optional, tag="4")]
	    pub info: Option<Info>,
	    /// DELTA coded
	    #[prost(sint64, repeated, tag="8")]
	    pub refs: Vec<i64>,
	}
	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct Relation {
	    #[prost(int64, required, tag="1")]
	    pub id: i64,
	    /// Parallel arrays.
	    #[prost(uint32, repeated, tag="2")]
	    pub keys: Vec<u32>,
	    #[prost(uint32, repeated, tag="3")]
	    pub vals: Vec<u32>,
	    #[prost(message, optional, tag="4")]
	    pub info: Option<Info>,
	    /// Parallel arrays
	    #[prost(int32, repeated, tag="8")]
	    pub roles_sid: Vec<i32>,
	    /// DELTA encoded
	    #[prost(sint64, repeated, tag="9")]
	    pub memids: Vec<i64>,
	    #[prost(enumeration="relation::MemberType", repeated, tag="10")]
	    pub types: Vec<i32>,
	}
	pub mod relation {
	    #[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
	    pub enum MemberType {
	        Node = 0,
	        Way = 1,
	        Relation = 2,
	    }
	}
}

pub mod fileformat {

	//protoc --java_out=../.. fileformat.proto

	//
	//  STORAGE LAYER: Storing primitives.
	//

	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct Blob {
	    /// No compression
	    #[prost(bytes, optional, tag="1")]
	    pub raw: Option<Vec<u8>>,
	    /// When compressed, the uncompressed size
	    #[prost(int32, optional, tag="2")]
	    pub raw_size: Option<i32>,
	    /// Possible compressed versions of the data.
	    #[prost(bytes, optional, tag="3")]
	    pub zlib_data: Option<Vec<u8>>,
	    /// PROPOSED feature for LZMA compressed data. SUPPORT IS NOT REQUIRED.
	    #[prost(bytes, optional, tag="4")]
	    pub lzma_data: Option<Vec<u8>>,
	    /// Formerly used for bzip2 compressed data. Depreciated in 2010.
	    /// Don't reuse this tag number.
	    #[prost(bytes, optional, tag="5")]
	    pub obsolete_bzip2_data: Option<Vec<u8>>,
	}
	// A file contains an sequence of fileblock headers, each prefixed by
	//their length in network byte order, followed by a data block
	//containing the actual data. types staring with a "_" are reserved.

	#[derive(Clone, Debug, PartialEq, Message)]
	pub struct BlobHeader {
	    #[prost(string, required, tag="1")]
	    pub type_: String,
	    #[prost(bytes, optional, tag="2")]
	    pub indexdata: Option<Vec<u8>>,
	    #[prost(int32, required, tag="3")]
	    pub datasize: i32,
	}
}