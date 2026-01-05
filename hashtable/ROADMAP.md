# 해시테이블 구현 로드맵

## 학습 목표

- 해시테이블의 내부 동작 원리를 직접 구현하며 이해한다
- Rust의 소유권/빌림 시스템을 통해 메모리 관리를 체험한다
- TDD 방식으로 점진적으로 기능을 확장한다

---

## 구현 단계 체크리스트

- [x] 1단계: 기본 구조체와 생성자
- [x] 2단계: 해시 함수
- [x] 3단계: put(key, value)
- [x] 4단계: get(key)
- [x] 5단계: 충돌 해결 (Chaining)
- [x] 6단계: delete(key)
- [x] 7단계: 동적 리사이징

---

## 단계별 상세

### 1단계: 기본 구조체와 생성자

**목표:** HashTable 구조체 정의 및 인스턴스 생성

**핵심 개념:**
- 고정 크기 배열(버킷)로 시작
- 각 버킷은 Option 타입으로 비어있음을 표현

**구현할 것:**
- `HashTable` 구조체
- `new()` 생성자

**테스트 케이스:**
```rust
#[test]
fn create_empty_hashtable() {
    let ht: HashTable<String, i32> = HashTable::new();
    // 생성 가능하면 성공
}
```

---

### 2단계: 해시 함수

**목표:** 키를 버킷 인덱스로 변환

**핵심 개념:**
- 해시 함수: 임의 크기 입력 → 고정 크기 출력
- 모듈로 연산으로 배열 범위 내 인덱스 생성
- Rust의 `std::hash::Hash` 트레이트 활용

**구현할 것:**
- `hash()` 내부 메서드

**테스트 케이스:**
```rust
#[test]
fn hash_returns_valid_index() {
    let ht: HashTable<String, i32> = HashTable::new();
    let index = ht.hash(&"hello".to_string());
    assert!(index < INITIAL_CAPACITY);
}
```

---

### 3단계: put(key, value)

**목표:** 키-값 쌍 저장

**핵심 개념:**
- 해시 → 인덱스 → 버킷에 저장
- 같은 키면 값 덮어쓰기

**구현할 것:**
- `put(&mut self, key: K, value: V)`

**테스트 케이스:**
```rust
#[test]
fn put_single_item() {
    let mut ht = HashTable::new();
    ht.put("name".to_string(), "alice".to_string());
    // 에러 없이 저장되면 성공
}
```

---

### 4단계: get(key)

**목표:** 키로 값 조회

**핵심 개념:**
- 해시 → 인덱스 → 버킷에서 조회
- 없으면 None 반환

**구현할 것:**
- `get(&self, key: &K) -> Option<&V>`

**테스트 케이스:**
```rust
#[test]
fn get_existing_key() {
    let mut ht = HashTable::new();
    ht.put("name".to_string(), "alice".to_string());
    assert_eq!(ht.get(&"name".to_string()), Some(&"alice".to_string()));
}

#[test]
fn get_nonexistent_key() {
    let ht: HashTable<String, String> = HashTable::new();
    assert_eq!(ht.get(&"name".to_string()), None);
}
```

---

### 5단계: 충돌 해결 (Chaining)

**목표:** 같은 인덱스에 여러 키-값 저장

**핵심 개념:**
- 해시 충돌: 서로 다른 키 → 같은 인덱스
- Chaining: 각 버킷을 연결 리스트(Vec)로 구성
- 삽입 시 리스트에 추가, 조회 시 리스트 순회

**구현할 것:**
- 버킷 구조 변경: `Option<(K, V)>` → `Vec<(K, V)>`
- `put`, `get` 수정

**테스트 케이스:**
```rust
#[test]
fn handle_collision() {
    let mut ht = HashTable::new();
    // 의도적으로 충돌을 일으키는 키들 삽입
    ht.put("key1".to_string(), "value1".to_string());
    ht.put("key2".to_string(), "value2".to_string());
    // 작은 capacity로 충돌 유도 후 둘 다 조회 가능한지 확인
    assert_eq!(ht.get(&"key1".to_string()), Some(&"value1".to_string()));
    assert_eq!(ht.get(&"key2".to_string()), Some(&"value2".to_string()));
}
```

---

### 6단계: delete(key)

**목표:** 키-값 쌍 삭제

**핵심 개념:**
- 해시 → 인덱스 → 버킷에서 해당 키 제거
- Chaining에서는 리스트에서 해당 항목 제거

**구현할 것:**
- `delete(&mut self, key: &K) -> Option<V>`

**테스트 케이스:**
```rust
#[test]
fn delete_existing_key() {
    let mut ht = HashTable::new();
    ht.put("name".to_string(), "alice".to_string());
    let deleted = ht.delete(&"name".to_string());
    assert_eq!(deleted, Some("alice".to_string()));
    assert_eq!(ht.get(&"name".to_string()), None);
}

#[test]
fn delete_nonexistent_key() {
    let mut ht: HashTable<String, String> = HashTable::new();
    let deleted = ht.delete(&"name".to_string());
    assert_eq!(deleted, None);
}
```

---

### 7단계: 동적 리사이징

**목표:** 적재율 초과 시 자동 확장

**핵심 개념:**
- 적재율(Load Factor) = 저장된 항목 수 / 버킷 수
- 보통 0.75 초과 시 리사이징
- 새 배열 할당 후 모든 항목 재해싱(rehash)

**구현할 것:**
- `len` 필드 추가 (저장된 항목 수)
- `resize()` 내부 메서드
- `put`에서 적재율 체크 후 `resize` 호출

**테스트 케이스:**
```rust
#[test]
fn resize_on_high_load_factor() {
    let mut ht = HashTable::new();
    // 많은 항목 삽입하여 리사이징 유도
    for i in 0..100 {
        ht.put(format!("key{}", i), i);
    }
    // 모든 항목이 여전히 조회 가능한지 확인
    for i in 0..100 {
        assert_eq!(ht.get(&format!("key{}", i)), Some(&i));
    }
}
```

---

## 참고 자료

### Rust 기초
- [The Rust Book](https://doc.rust-lang.org/book/) - 공식 가이드
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 예제 중심 학습

### 해시테이블 이론
- [Wikipedia: Hash Table](https://en.wikipedia.org/wiki/Hash_table)
- [Visualgo: Hash Table](https://visualgo.net/en/hashtable) - 시각화

### Rust 해시 관련
- [std::hash 모듈](https://doc.rust-lang.org/std/hash/index.html)
- [std::collections::HashMap 소스코드](https://doc.rust-lang.org/src/std/collections/hash/map.rs.html)
