use crate::error::*;

/// Decoder of `相手端末からの状態通知`  
/// <https://mono-wireless.com/jp/products/TWE-APPS/App_Twelite/step3-81.html>
#[derive(Debug)]
pub struct StatusNotify {
    buf: [u8; 24],
}

fn char2bin(c: u8) -> Result<u8, DecodeError> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        c => Err(DecodeError::InvalidCharacter(c)),
    }
}

impl StatusNotify {
    /// Decode value by byte array reference.
    ///
    /// WARNING: This method doesn't check the validity.
    /// See [`StatusNotify::validate`], If you need validator.
    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        let len = buf.len();

        if len != ":7881150175810000380026C9000C04220000FFFFFFFFFFA7".len() {
            return Err(DecodeError::InvalidLength(len));
        }

        if buf[0] != b':' {
            return Err(DecodeError::InvalidCharacter(buf[0]));
        }

        let buf = &buf[1..];

        let mut out = Self {
            buf: Default::default(),
        };

        for (n, out) in out.buf.iter_mut().enumerate() {
            *out |= char2bin(buf[n * 2])? << 4;
            *out |= char2bin(buf[n * 2 + 1])?;
        }

        Ok(out)
    }

    /// Decode value by [`&str`].
    ///
    /// WARNING: This method doesn't check the validity.
    /// See [`StatusNotify::validate`], If you need validator.
    pub fn decode_str(buf: &str) -> Result<Self, DecodeError> {
        Self::decode(buf.as_bytes())
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///  ^^
    /// ```
    ///
    /// # Official Reference
    /// 相手端末の論理デバイスID（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.source_device_id(), 0x78);
    /// ```
    ///
    /// 相手端末の論理デバイスIDが78であることを表します。
    pub fn source_device_id(&self) -> u8 {
        self.buf[0]
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///    ^^
    /// ```
    ///
    /// # Official Reference
    /// データの種別（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.command(), 0x81);
    /// ```
    ///
    /// 81はデータが相手端末の状態であることを表します。
    pub fn command(&self) -> u8 {
        self.buf[1]
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///      ^^
    /// ```
    ///
    /// # Official Reference
    /// パケット識別子（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.packet_id(), 0x15);
    /// ```
    ///
    /// システムが使用する値です。
    pub fn packet_id(&self) -> u8 {
        self.buf[2]
    }

    /// If you need validate protocol version, you can use [`StatusNotify::validate_protocol_version`].
    ///
    /// # Official Reference
    /// プロトコルバージョン（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.protocol_version(), 0x01);
    /// ```
    ///
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///        ^^
    /// ```
    pub fn protocol_version(&self) -> u8 {
        self.buf[3]
    }

    /// If you need a PdBi value, you can use [`StatusNotify::lqi_dbm`].
    ///
    /// # Official Reference
    /// 受信電波品質（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.lqi(), 0x75);
    /// ```
    /// 受信電波品質が0x75であることを表します。（値は0～0xFF）  
    /// 受信電波品質（LQI）は0から255までの数値で表されます。  
    /// あくまでも目安としてですが、以下の計算式でdBmに変換できます。  
    /// `PdBm=(7*LQI-1970)/20`  
    /// 例：LQI（(0-255)が125の時、約-54.75dBm）
    ///
    /// 50未満（悪い -80dbm 未満）、50～100（やや悪い）、100～150（良好）、
    /// 150以上（アンテナの近傍）といった区分けが品質の目安になります。
    /// あくまでも目安ですので、実地での検証をしてください。
    pub fn lqi(&self) -> u8 {
        self.buf[4]
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///            ^^^^^^^^
    /// ```
    ///
    /// # Official Reference
    /// 相手端末の個体識別番号（4バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.hardware_id(), 0x81000038);
    /// ```
    ///
    /// 相手端末の個体識別番号が0x81000038であることを表します。  
    /// 製造時に一意に割り当てられる値です。変更することは出来ません。
    pub fn hardware_id(&self) -> u32 {
        u32::from_be_bytes([self.buf[5], self.buf[6], self.buf[7], self.buf[8]])
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                    ^^
    /// ```
    ///
    /// # Official Reference
    /// 宛先端末の論理デバイスID（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.dest_device_id(), 0);
    /// ```
    ///
    /// 宛先端末の論理デバイスIDが00であることを表します。 
    pub fn dest_device_id(&self) -> u8 {
        self.buf[9]
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                      ^^^^
    /// ```
    ///
    /// # Official Reference
    /// タイムスタンプ（2バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.timestamp(), 0x26c9);
    /// ```
    ///
    /// タイムスタンプです。1/64秒でカウントアップします。0xFFFFで0に戻ります。 
    pub fn timestamp(&self) -> u16 {
        u16::from_be_bytes([self.buf[10], self.buf[11]])
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                          ^^
    /// ```
    ///
    /// # Official Reference
    /// 中継フラグ（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.relay_count(), 0);
    /// ```
    ///
    /// 中継フラグは中継された回数を示します。
    /// 00は中継されていないことを表します。
    /// 中継が一回入った場合は01になります。
    /// 中継回数は最大で３回です。
    /// 中継回数の初期値は１です。
    /// 中継回数はインタラクティブモードのオプションビットで最大３まで変更できます。
    pub fn relay_count(&self) -> u8 {
        self.buf[12]
    }

    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                            ^^^^
    /// ```
    ///
    /// # Official Reference
    /// 電源電圧（2バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.power_voltage_millis(), 3076); // [mV]
    /// ```
    ///
    /// 電源電圧が0x0C04（3.076V）であることを表します。  
    /// 値はmVです。
    pub fn power_voltage_millis(&self) -> u16 {
        u16::from_be_bytes([self.buf[13], self.buf[14]])
    }


    /// If you need separated value, you can use likes [`StatusNotify::di1_status`].
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                  ^^
    /// ```
    ///
    /// # Official Reference
    /// デジタル入力（DI）の値（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.di_status(), 0);
    /// ```
    ///
    /// デジタル入力（DI）の値を表します。
    /// 値はDI1(0x1) DI2(0x2) DI3(0x4) DI4(0x8)に対応しています。
    /// 例えばDI1に接続されたスイッチが押されている場合、値は01となります。
    /// DI4のスイッチが押されている場合、値は08となります。
    /// 全てのスイッチが押されている場合、値は0Fとなります。
    pub fn di_status(&self) -> u8 {
        self.buf[16]
    }

    /// If you need separated value, you can use likes [`StatusNotify::di1_changed`].
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                    ^^
    /// ```
    ///
    /// # Official Reference
    /// デジタル入力（DI）の変更状態値（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.di_changed(), 0);
    /// ```
    ///
    /// デジタル入力（DI）の変更状態値を表します。
    pub fn di_changed(&self) -> u8 {
        self.buf[17]
    }

    /// If you need A/D value in milli-votage, you can use [`StatusNotify::ad1_voltage_millis`].
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                      ^^
    /// ```
    ///
    /// # Official Reference
    /// アナログ入力（AI）の値（4バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.ad4_value(), 0xFF);
    /// assert_eq!(notify.ad3_value(), 0xFF);
    /// assert_eq!(notify.ad2_value(), 0xFF);
    /// assert_eq!(notify.ad1_value(), 0xFF);
    /// ```
    ///
    /// アナログ入力（AI）のAI4、AI3、AI2、AI1（各1バイト）の変換値を表します。
    /// 入力電圧0～2000[mV]のAI値を16で割った値です。（入力電圧が2000mV以上の場合は無効になります。）
    pub fn ad4_value(&self) -> u8 {
        self.buf[18]
    }

    /// See [`StatusNotify::ad4_value`].
    ///
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                        ^^
    /// ```
    pub fn ad3_value(&self) -> u8 {
        self.buf[19]
    }

    /// See [`StatusNotify::ad4_value`].
    ///
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                          ^^
    /// ```
    pub fn ad2_value(&self) -> u8 {
        self.buf[20]
    }

    /// See [`StatusNotify::ad4_value`].
    ///
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                            ^^
    /// ```
    pub fn ad1_value(&self) -> u8 {
        self.buf[21]
    }

    /// If you need A/D value in milli-votage, you can use [`StatusNotify::ad1_voltage_millis`].
    /// If you need separated value, you can use [`StatusNotify::ad1_fix`].
    ///
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                              ^^
    /// ```
    ///
    /// # Official Reference
    /// アナログ入力（AI）の補正値（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.ad_fix(), 0xFF);
    /// ```
    ///
    /// アナログ入力（AI）の補正値を表します。  
    /// アナログ値の復元には以下のように計算してください。  
    /// `AI[mV] = （アナログ入力の値 * 4 + 補正値） * 4`
    ///
    /// AD1～AD4の補正値　（LSBから順に２ビットずつ補正値、LSB側が　AD1, MSB側が AD4）
    pub fn ad_fix(&self) -> u8 {
        self.buf[22]
    }

    /// If you need validate checksum, you can use [`StatusNotify::validate_checksum`].
    ///
    /// # Byte position
    /// ```txt
    /// :7881150175810000380026C9000C04220000FFFFFFFFFFA7
    ///                                                ^^
    /// ```
    ///
    /// # Official Reference
    /// チェックサム（1バイト）
    ///
    /// ```
    /// # use twelite_serial::StatusNotify;
    /// # let notify = StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();
    /// # notify.validate().unwrap();
    /// assert_eq!(notify.checksum(), 0xA7);
    /// ```
    ///
    /// チェックサムが0xA7であることを表します。
    pub fn checksum(&self) -> u8 {
        self.buf[23]
    }

    /// See [`StatusNotify::lqi`].
    pub fn lqi_dbm(&self) -> f32 {
        (7 * self.lqi() as i32 - 1970) as f32 / 20.0
    }

    /// Represent digital input status as [`bool`]. See [`StatusNotify::di_status`].
    pub fn di1_status(&self) -> bool {
        (self.di_status() & (1 << 0)) != 0
    }

    /// Represent digital input status as [`bool`]. See [`StatusNotify::di_status`].
    pub fn di2_status(&self) -> bool {
        (self.di_status() & (1 << 1)) != 0
    }

    /// Represent digital input status as [`bool`]. See [`StatusNotify::di_status`].
    pub fn di3_status(&self) -> bool {
        (self.di_status() & (1 << 2)) != 0
    }

    /// Represent digital input status as [`bool`]. See [`StatusNotify::di_status`].
    pub fn di4_status(&self) -> bool {
        (self.di_status() & (1 << 3)) != 0
    }

    /// See [`StatusNotify::di_changed`].
    pub fn di1_changed(&self) -> bool {
        (self.di_changed() & (1 << 0)) != 0
    }

    /// See [`StatusNotify::di_changed`].
    pub fn di2_changed(&self) -> bool {
        (self.di_changed() & (1 << 1)) != 0
    }

    /// See [`StatusNotify::di_changed`].
    pub fn di3_changed(&self) -> bool {
        (self.di_changed() & (1 << 2)) != 0
    }

    /// See [`StatusNotify::di_changed`].
    pub fn di4_changed(&self) -> bool {
        (self.di_changed() & (1 << 3)) != 0
    }

    /// See [`StatusNotify::ad1_value`].
    pub fn ad_value(&self) -> [u8; 4] {
        [
            self.ad1_value(),
            self.ad2_value(),
            self.ad3_value(),
            self.ad4_value(),
        ]
    }

    /// See [`StatusNotify::ad_fix`].
    pub fn ad1_fix(&self) -> u8 {
        #![allow(clippy::erasing_op)]
        #![allow(clippy::identity_op)]
        (self.ad_fix() >> (2 * 0)) & 0b11
    }

    /// See [`StatusNotify::ad_fix`].
    pub fn ad2_fix(&self) -> u8 {
        #![allow(clippy::identity_op)]
        (self.ad_fix() >> (2 * 1)) & 0b11
    }

    /// See [`StatusNotify::ad_fix`].
    pub fn ad3_fix(&self) -> u8 {
        (self.ad_fix() >> (2 * 2)) & 0b11
    }

    /// See [`StatusNotify::ad_fix`].
    pub fn ad4_fix(&self) -> u8 {
        (self.ad_fix() >> (2 * 3)) & 0b11
    }

    /// See [`StatusNotify::ad1_value`] and [`StatusNotify::ad_fix`].
    pub fn ad1_voltage_millis(&self) -> u16 {
        let value = self.ad1_value() as u16;
        let fix = self.ad1_fix() as u16;
        (value * 4 + fix) * 4
    }

    /// See [`StatusNotify::ad1_value`] and [`StatusNotify::ad_fix`].
    pub fn ad2_voltage_millis(&self) -> u16 {
        let value = self.ad2_value() as u16;
        let fix = self.ad2_fix() as u16;
        (value * 4 + fix) * 4
    }

    /// See [`StatusNotify::ad1_value`] and [`StatusNotify::ad_fix`].
    pub fn ad3_voltage_millis(&self) -> u16 {
        let value = self.ad3_value() as u16;
        let fix = self.ad3_fix() as u16;
        (value * 4 + fix) * 4
    }

    /// See [`StatusNotify::ad1_value`] and [`StatusNotify::ad_fix`].
    pub fn ad4_voltage_millis(&self) -> u16 {
        let value = self.ad4_value() as u16;
        let fix = self.ad4_fix() as u16;
        (value * 4 + fix) * 4
    }

    /// Check the checksum.
    ///
    /// If unexpected value is comming, the value sends as Err(u8).
    ///
    /// If you need validate totally, you can use [`StatusNotify::validate`].
    ///
    /// # Official Reference
    /// チェックサムとは受け取ったデータが正しいかどうかを確認するための付加データです。
    ///
    /// データ部の各バイトの和を８ビット幅で計算し２の補数をとります。
    /// つまりデータ部の各バイトの総和＋チェックサムバイトを８ビット幅で計算すると０になります。
    ///
    /// チェックサムバイトをアスキー文字列２文字で表現します。
    ///
    /// 例えば 00A01301FF123456 では 0x00 + 0xA0 + ... + 0x56 = 0x4F となり、
    /// この二の補数は0xB1 です。（つまり 0x4F + 0xB1 = 0）
    pub fn validate_checksum(&self) -> Result<(), u8> {
        let checksum = self.buf.iter().fold(0u8, |s, v| s.wrapping_add(*v));
        (checksum == 0).then_some(()).ok_or(checksum)
    }

    /// Check the protocol version is equal to `0x01`.
    ///
    /// If unexpected value is comming, the value sends as Err(u8).
    ///
    /// If you need validate totally, you can use [`StatusNotify::validate`].
    pub fn validate_protocol_version(&self) -> Result<(), u8> {
        let version = self.protocol_version();
        (version == 0x01).then_some(()).ok_or(version)
    }

    /// Check the command value is equal to `0x81`.
    ///
    /// If unexpected value is comming, the value sends as Err(u8).
    ///
    /// If you need validate totally, you can use [`StatusNotify::validate`].
    pub fn validate_command(&self) -> Result<(), u8> {
        let command = self.command();
        (command == 0x81).then_some(()).ok_or(command)
    }


    /// Check the relay count is less or equal than `3`.
    ///
    /// If unexpected value is comming, the value sends as Err(u8).
    ///
    /// If you need validate totally, you can use [`StatusNotify::validate`].
    pub fn validate_relay_count(&self) -> Result<(), u8> {
        let relay_count = self.relay_count();
        (relay_count <= 3).then_some(()).ok_or(relay_count)
    }

    /// Validate totally.
    pub fn validate(&self) -> Result<(), ValidateError> {
        self.validate_checksum()
            .map_err(ValidateError::InvalidChecksum)?;

        self.validate_protocol_version()
            .map_err(ValidateError::InvalidProtocolVersion)?;

        self.validate_command()
            .map_err(ValidateError::InvalidCommand)?;

        self.validate_relay_count()
            .map_err(ValidateError::InvalidRelayCount)?;

        Ok(())
    }

    /// Get reference of raw value.
    pub fn as_bytes(&self) -> &[u8; 24] {
        &self.buf
    }

    /// Drop out to raw value.
    pub fn into_bytes(self) -> [u8; 24] {
        self.buf
    }
}

#[test]
fn test() {
    // Case 1
    let notify =
        StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFA7").unwrap();

    assert_eq!(notify.source_device_id(), 0x78);
    assert_eq!(notify.command(), 0x81);
    assert_eq!(notify.packet_id(), 0x15);
    assert_eq!(notify.protocol_version(), 0x01);
    assert_eq!(notify.lqi(), 0x75);
    assert_eq!(notify.hardware_id(), 0x81000038);
    assert_eq!(notify.dest_device_id(), 0x00);
    assert_eq!(notify.timestamp(), 0x26c9);
    assert_eq!(notify.relay_count(), 0x00);
    assert_eq!(notify.power_voltage_millis(), 0x0c04);
    assert_eq!(notify.di_status(), 0x00);
    assert_eq!(notify.di_changed(), 0x00);
    assert_eq!(notify.ad_value(), [0xff, 0xff, 0xff, 0xff]);
    assert_eq!(notify.ad_fix(), 0xff);
    assert_eq!(notify.checksum(), 0xa7);
    assert_eq!(Ok(()), notify.validate());


    // Invalid Checksum
    let notify =
        StatusNotify::decode_str(":7881150175810000380026C9000C04220000FFFFFFFFFFFF").unwrap();

    assert_eq!(Err(ValidateError::InvalidChecksum(0xFFu8 - 0xA7)), notify.validate());


    // Invalid Protocol Version
    let notify =
        StatusNotify::decode_str(":7881150075810000380026C9000C04220000FFFFFFFFFFA8").unwrap();

    assert_eq!(Err(ValidateError::InvalidProtocolVersion(0x00)), notify.validate());


    // Invalid Command
    let notify =
        StatusNotify::decode_str(":7880150175810000380026C9000C04220000FFFFFFFFFFA8").unwrap();

    assert_eq!(Err(ValidateError::InvalidCommand(0x80)), notify.validate());


    // Invalid Realy Count
    let notify =
        StatusNotify::decode_str(":7881150175810000380026C9FF0C04220000FFFFFFFFFFA8").unwrap();

    assert_eq!(Err(ValidateError::InvalidRelayCount(0xFF)), notify.validate());
}
