/// Тип данных в байтовом буфере (WireType).
enum WireType {
    /// Тип Varint  обозначает одно значение VARINT.
    Varint,
    // Тип I64 это 8 байтов в формате little-endian содержащие
    // 64-битовое целое значение  со знаком или значение типа double.
    //I64,  -- не требуется для этого задания
    /// Тип Len обозначает длину в формате VARINT за которым следует указанное количество байтов
    Len,
    // Тип I32 это 4 байтоа в формате little-endian содержащие
    // 32-битовое целое значение  со знаком или значение типа float.
    //I32,  -- не требуется для этого задания
}

#[derive(Debug)]
/// Тип поля, typed based on the wire type.
enum FieldValue<'a> {
    Varint(u64),
    //I64(i64),  -- не требуется для этого задания
    Len(&'a [u8]),
    //I32(i32),  -- не требуется для этого задания
}

#[derive(Debug)]
/// Содержит номер поля и его значение.
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default {
    fn add_field(&mut self, field: Field<'a>);
}

impl From<u64> for WireType {
    fn from(value: u64) -> Self {
        match value {
            0 => WireType::Varint,
            //1 => WireType::I64,  -- не требуется для этого задания
            2 => WireType::Len,
            //5 => WireType::I32,  -- не требуется для этого задания
            _ => panic!("Invalid wire type: {value}"),
        }
    }
}

impl<'a> FieldValue<'a> {
    fn as_str(&self) -> &'a str {
        let FieldValue::Len(data) = self else {
            panic!("Expected string to be a `Len` field");
        };
        std::str::from_utf8(data).expect("Invalid string")
    }

    fn as_bytes(&self) -> &'a [u8] {
        let FieldValue::Len(data) = self else {
            panic!("Expected bytes to be a `Len` field");
        };
        data
    }

    fn as_u64(&self) -> u64 {
        let FieldValue::Varint(value) = self else {
            panic!("Expected `u64` to be a `Varint` field");
        };
        *value
    }
}

/// Обрабатывает VARINT, возвращает значение и оставшиеся байты.
fn parse_varint(data: &[u8]) -> (u64, &[u8]) {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            panic!("Not enough bytes for varint");
        };
        if b & 0x80 == 0 {
            // Это последний байт VARINT, преобразуем число
            // в u64 и возвращаем.
            let mut value = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return (value, &data[i + 1..]);
        }
    }

    // Больше 7 байтов - ошибка.
    panic!("Слишком много байтов для varint");
}

/// Преобразует тег в номер поля и WireType.
fn unpack_tag(tag: u64) -> (u64, WireType) {
    let field_num = tag >> 3;
    let wire_type = WireType::from(tag & 0x7);
    (field_num, wire_type)
}


/// Обрабатывает поле, возвращает оставшиеся байты
fn parse_field(data: &[u8]) -> (Field, &[u8]) {
    // Разбираем тег (поле), используя функцию parse_varint
    let (tag, remainder) = parse_varint(data);
    // Разбираем номер поля и тип данных (WireType) из тега
    let (field_num, wire_type) = unpack_tag(tag);

    // В зависимости от типа данных (wire_type) обрабатываем поле
    let (field_value, remainder) = match wire_type {
        // Если тип данных Varint, то разбираем значение varint
        WireType::Varint => {
            // Разбираем значение varint
            let (value, remainder) = parse_varint(remainder);
            // Возвращаем поле с типом FieldValue::Varint и оставшиеся байты
            (FieldValue::Varint(value), remainder)
        }
        // Если тип данных Len, то разбираем длину данных
        WireType::Len => {
            // Разбираем длину данных (с помощью varint)
            let (length, remainder) = parse_varint(remainder);
            // Получаем сами данные длиной length
            let len_data = &remainder[..length as usize];
            // Возвращаем поле с типом FieldValue::Len и оставшиеся байты
            (FieldValue::Len(len_data), &remainder[length as usize..])
        }
    };

    // Создаем объект Field с номером поля и значением, и возвращаем его вместе с оставшимися байтами
    (Field { field_num, value: field_value }, remainder)
}

/// Обрабатывает сообщение data, вызывая `T::add_field` для каждого поля
/// в сообщении.
///
/// Обрабатывается весь входной буфер.
fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> T {
    let mut result = T::default();
    while !data.is_empty() {
        let parsed = parse_field(data);
        result.add_field(parsed.0);
        data = parsed.1;
    }
    result
}

#[derive(Debug, Default, PartialEq)]
struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

#[derive(Debug, Default, PartialEq)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

/// ProtoMessage для PhoneNumber
impl<'a> ProtoMessage<'a> for PhoneNumber<'a> {
    // Метод добавления поля в структуру PhoneNumber
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            // Если поле с номером 1 (номер телефона)
            1 => {
                // Проверяем, что значение поля соответствует типу Len (строка)
                if let FieldValue::Len(data) = field.value {
                    // Преобразуем байтовые данные в строку (номер телефона)
                    self.number = std::str::from_utf8(data).expect("Invalid string for number");
                }
            }
            // Преобразуем байтовые данные в строку (номер телефона)
            2 => {
                // Проверяем, что значение поля соответствует типу Len (строка)
                if let FieldValue::Len(data) = field.value {
                    // Преобразуем байтовые данные в строку (тип телефона)
                    self.type_ = std::str::from_utf8(data).expect("Invalid string for type");
                }
            }
            // Если поле с неизвестным номером
            _ => panic!("Unknown field number for PhoneNumber"),
        }
    }
}

/// Реализация ProtoMessage для Person
impl<'a> ProtoMessage<'a> for Person<'a> {
    // Метод добавления поля в структуру Person
    fn add_field(&mut self, field: Field<'a>) {
        match field.field_num {
            // Если поле с номером 1 (имя человека)
            1 => {
                // Проверяем, что значение поля соответствует типу Len (строка)
                if let FieldValue::Len(data) = field.value {
                    // Преобразуем байтовые данные в строку (имя)
                    self.name = std::str::from_utf8(data).expect("Invalid string for name");
                }
            }
            // Если поле с номером 2 (идентификатор человека)
            2 => {
                // Проверяем, что значение поля соответствует типу Varint (целое число)
                if let FieldValue::Varint(value) = field.value {
                    // Присваиваем идентификатор
                    self.id = value;
                }
            }
            // Если поле с номером 3 (номер телефона)
            3 => {
                // Проверяем, что значение поля соответствует типу Len (сериализованные данные для телефона)
                if let FieldValue::Len(data) = field.value {
                    // Преобразуем данные в структуру PhoneNumber с помощью parse_message
                    let phone_number: PhoneNumber = parse_message(data);
                    // Добавляем номер телефона в список
                    self.phone.push(phone_number);
                }
            }
            // Если поле с неизвестным номером
            _ => panic!("Unknown field number for Person"),
        }
    }
}

fn main() {
    let person_id: Person = parse_message(&[0x10, 0x2a]);
    assert_eq!(person_id, Person { name: "", id: 42, phone: vec![] });

    let person_name: Person = parse_message(&[
        0x0a, 0x0e, 0x62, 0x65, 0x61, 0x75, 0x74, 0x69, 0x66, 0x75, 0x6c, 0x20,
        0x6e, 0x61, 0x6d, 0x65,
    ]);
    assert_eq!(person_name, Person { name: "beautiful name", id: 0, phone: vec![] });

    let person_name_id: Person =
        parse_message(&[0x0a, 0x04, 0x45, 0x76, 0x61, 0x6e, 0x10, 0x16]);
    assert_eq!(person_name_id, Person { name: "Evan", id: 22, phone: vec![] });

    let phone: Person = parse_message(&[
        0x0a, 0x00, 0x10, 0x00, 0x1a, 0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x33,
        0x34, 0x2d, 0x37, 0x37, 0x37, 0x2d, 0x39, 0x30, 0x39, 0x30, 0x12, 0x04,
        0x68, 0x6f, 0x6d, 0x65,
    ]);
    assert_eq!(
        phone,
        Person {
            name: "",
            id: 0,
            phone: vec![PhoneNumber { number: "+1234-777-9090", type_: "home" },],
        }
    );

    // Put that all together into a single parse.
    let person: Person = parse_message(&[
        0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a,
        0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35,
        0x2d, 0x31, 0x32, 0x31, 0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a,
        0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d, 0x38, 0x36, 0x37,
        0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
        0x65,
    ]);
    assert_eq!(
        person,
        Person {
            name: "maxwell",
            id: 42,
            phone: vec![
                PhoneNumber { number: "+1202-555-1212", type_: "home" },
                PhoneNumber { number: "+1800-867-5308", type_: "mobile" },
            ]
        }
    );
}