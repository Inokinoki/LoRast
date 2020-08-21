use iced::{button, text_input,
    Align, Column, Row, Element, Sandbox, Settings, Length,
    Button, TextInput, Text, Checkbox};

pub fn main() {
    LoRaParam::run(Settings::default())
}

#[derive(Default)]
struct LoRaParam {
    // spreading_factor: i8,   // SF
    // coding_rate: i8,        // CR
    // bandwidth: i32,
    // payload_length: i32,
    // preamble_length: i32,
    has_header: bool,
    has_crc: bool,
    enable_low_data_rate_optimization: bool,

    payload_length_buffer: String,

    input: text_input::State,

    /* widgets */
    crcCheckbox: Checkbox,
    ldrCheckbox: Checkbox,
    headerCheckbox: Checkbox,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    HeaderCheckboxToggled(bool),
    CRCCheckboxToggled(bool),
    LDRCheckboxToggled(bool),
}

impl Sandbox for LoRaParam {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("LoRa Air Time Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(buffer) => {
                println!("{}", buffer);

                self.payload_length_buffer = buffer;
            },
            Message::HeaderCheckboxToggled(enabled) => {
                self.has_header = enabled;

                println!("{}", enabled);
            },
            Message::CRCCheckboxToggled(enabled) => {
                self.has_crc = enabled;

                println!("{}", enabled);
            },
            Message::LDRCheckboxToggled(enabled) => {
                self.enable_low_data_rate_optimization = enabled;

                println!("{}", enabled);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        self.headerCheckbox = Checkbox::new(
            true,
            "With CRC",
            Message::CRCCheckboxToggled,
        );
        self.headerCheckbox.width(Length::from(200));
    
        self.crcCheckbox = Checkbox::new(
            true,
            "With Header",
            Message::HeaderCheckboxToggled,
        );
        self.crcCheckbox.width(Length::from(200));

        self.ldrCheckbox = Checkbox::new(
            false,
            "Enable Low Data Rate",
            Message::LDRCheckboxToggled,
        );
        self.ldrCheckbox.width(Length::from(200));

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                TextInput::new(
                        &mut self.input,
                        "Type something...",
                        &self.payload_length_buffer,
                        Message::InputChanged,
                )
            )
            .push(
                Row::new().padding(20).align_items(Align::Center)
                .push(self.headerCheckbox)
                .push(self.crcCheckbox)
                .push(self.ldrCheckbox)
            )
            .into()
    }
}
