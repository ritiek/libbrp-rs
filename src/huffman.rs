#[derive(Debug, Clone, Copy)]
struct Node {
    left_child: i16,
    right_child: i16,
    parent: u8,
    bits: u8,
    val: u16,
    frequency: i32,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            left_child: -1,
            right_child: -1,
            parent: 0,
            bits: 0,
            val: 0,
            frequency: 0,
        }
    }
}

pub fn build() {
    let g_freqs: Vec<i32> = vec![
      101342, 9667, 3497, 1072, 0, 3793, 0, 0, 2815, 5235, 0, 0, 0, 3570, 0, 0,
      0,      1383, 0,    0,    0, 2970, 0, 0, 2857, 0,    0, 0, 0, 0,    0, 0,
      0,      1199, 0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 1494,
      1974,   0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 1351, 0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 1475,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0,
      0,      0,    0,    0,    0, 0,    0, 0, 0,    0,    0, 0, 0, 0,    0, 0
    ];

    let mut nodes_ = Vec::<Node>::new();
    for i in 0i32..511 {
        let mut node = Node::default();
        nodes_.push(node);
    }
    for i in 0i32..256 {
        nodes_[i as usize].frequency = g_freqs[i as usize];
    }

    let mut node_count: i32 = 256;

    let mut smallest1: i32;
    let mut smallest2: i32;

    while node_count < 511 {
        let mut i: i32 = 0;

        while nodes_[i as usize].parent != 0 {
            i += 1;
        }
        smallest1 = i;
        i += 1;
        while nodes_[i as usize].parent != 0 {
            i += 1;
        }
        smallest2 = i;
        i += 1;
        while i < node_count {
            if nodes_[i as usize].parent == 0 {
                if nodes_[smallest1 as usize].frequency > nodes_[smallest2 as usize].frequency {
                    if nodes_[i as usize].frequency < nodes_[smallest1 as usize].frequency {
                        smallest1 = i;
                    }
                } else {
                    if nodes_[i as usize].frequency < nodes_[smallest2 as usize].frequency {
                        smallest2 = i;
                    }
                }
            }
            i += 1;
        }
        nodes_[(node_count) as usize].frequency = nodes_[smallest1 as usize].frequency + nodes_[smallest2 as usize].frequency;
        nodes_[smallest1 as usize].parent = (node_count - 255) as u8;
        nodes_[smallest2 as usize].parent = (node_count - 255) as u8;
        nodes_[(node_count) as usize].left_child = smallest1 as i16;
        nodes_[(node_count) as usize].right_child = smallest2 as i16;

        node_count += 1;
    }

    assert!(nodes_[509].parent != 0);
    assert!(nodes_[510].parent == 0);

    for i in 0i32..256 {
        nodes_[i as usize].val = 0;
        nodes_[i as usize].bits = 0;
        let mut index: i32 = i;
        while nodes_[i as usize].parent != 0 {
            if nodes_[(nodes_[i as usize].parent + 255) as usize].right_child == index as i16 {
                nodes_[i as usize].val = (nodes_[i as usize].val << 1 | 0x01) as u16;
            } else {
                assert!(nodes_[(nodes_[i as usize].parent + 255) as usize].left_child == index as i16);
                nodes_[i as usize].val = nodes_[i as usize].val << 1;
            }
            nodes_[i as usize].bits += 1;

            index = (nodes_[i as usize].parent + 255) as i32;
        }
        if nodes_[i as usize].bits >= 8 {
            nodes_[i as usize].bits = 8;
            nodes_[i as usize].val = (i << 1) as u16;
        } else {
            nodes_[i as usize].val = (nodes_[i as usize].val << 1 | 0x01) as u16;
        }
        nodes_[i as usize].bits += 1;
    }
}
