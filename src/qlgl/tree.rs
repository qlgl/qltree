#![allow(dead_code)]
use std::fs::{ File, OpenOptions };
use std::io::Read;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{ Arc, Mutex };

use super::{ bytes_ext::BytesExtension, nodes::Node, Index, Value };


pub struct Tree<I: Index, V: Value> {
  node_name: String,                          // TYPE : 트리의 이름
  root: Arc<Mutex<Node<I, V>>>,               // TYPE : 트리의 루트 노드
  node_file: Arc<Mutex<File>>,               // TYPE : 노드 파일
  phantom: PhantomData<(I, V)>,               // TYPE : 트리의 인덱스와 값의 타입을 저장
}

impl<I: Index, V: Value> Tree<I,V> {
  pub fn new(base_path: PathBuf, node_name: &str) -> Self {
    if !base_path.exists() { std::fs::create_dir_all(&base_path).unwrap(); }
    if !base_path.is_dir() { panic!("base_path is not directory"); }

    let file_path = base_path.join("tree.ql");

    let mut node_file = if !file_path.exists() {
      let mut node_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)
        .unwrap();

      let mut root = Node::<I, V>::new();
      root.set_root();
      root.set_dirty();
      root.set_offset(0);
      root.set_leaf(None);
      root.set_parent_offset(0);
      root.write(&mut node_file).unwrap();

      node_file
    } else {
      let node_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_path)
        .unwrap();
      node_file
    };

    let mut root = Node::<I, V>::new();
    root.set_offset(0);

    let mut bufs: Vec<u8> = vec![0; Node::<I, V>::get_byte_size()];
    node_file.read_exact(&mut bufs).unwrap();

    root.from_bytes(bufs).unwrap();


    Self {
      node_name: node_name.to_string(),
      root: Arc::new(Mutex::new(root)),
      node_file: Arc::new(Mutex::new(node_file)),

      phantom: PhantomData,
    }
  }
  pub fn insert(&mut self, index: I, value: V) -> Result<(), String> {
    let mut root = self.root.lock().unwrap();
    let mut node_file = self.node_file.lock().unwrap();

    root.insert(&mut node_file, index, value).unwrap();

    Ok(())
  }
} 
