const int DELAY = 100;
const int POTS = 4;
int potValues[] = {0, 0, 0, 0, 0, 0};
int potIds[] = {0, 1, 2, 3};

int s0 = 5;
int s1 = 4;
int s2 = 3;
int s3 = 2;

int SIG_pin = A7;

void setup() {
	pinMode(s0, OUTPUT);
	pinMode(s1, OUTPUT);
	pinMode(s2, OUTPUT);
	pinMode(s3, OUTPUT);

	digitalWrite(s0, LOW);
	digitalWrite(s1, LOW);
	digitalWrite(s2, LOW);
	digitalWrite(s3, LOW);
	Serial.begin(115200);
}

void loop() {
	for (int i = 0; i < POTS; i++) {
		int value = readMux(potIds[i]);

    // sometimes my pots hit 99 but don't go up to 100 :(
		if (value == 99) {
			value = 100;
		}
		potValues[i] = value;
		Serial.print("pot");
		Serial.print(i);
		Serial.print("=");
		Serial.println(value);
	}
	Serial.print("but0=");
	Serial.println(readMuxRaw(4) == 1023);

	Serial.print("but1=");
	Serial.println(readMuxRaw(5) == 1023);

	Serial.print("but2=");
	Serial.println(readMuxRaw(6) == 1023);

	Serial.print("but3=");
	Serial.println(readMuxRaw(7) == 1023);

	Serial.print("but4=");
	Serial.println(readMuxRaw(8) == 1023);

	delay(DELAY);
}

int readMuxRaw(int channel) {
	int controlPin[] = {s0, s1, s2, s3};

	int muxChannel[16][4] = {
		{0, 0, 0, 0}, // channel 0
		{1, 0, 0, 0}, // channel 1
		{0, 1, 0, 0}, // channel 2
		{1, 1, 0, 0}, // channel 3
		{0, 0, 1, 0}, // channel 4
		{1, 0, 1, 0}, // channel 5
		{0, 1, 1, 0}, // channel 6
		{1, 1, 1, 0}, // channel 7
		{0, 0, 0, 1}, // channel 8
		{1, 0, 0, 1}, // channel 9
		{0, 1, 0, 1}, // channel 10
		{1, 1, 0, 1}, // channel 11
		{0, 0, 1, 1}, // channel 12
		{1, 0, 1, 1}, // channel 13
		{0, 1, 1, 1}, // channel 14
		{1, 1, 1, 1}  // channel 15
	};

	// loop through the 4 sig
	for (int i = 0; i < 4; i++) {
		digitalWrite(controlPin[i], muxChannel[channel][i]);
	}

	int val = analogRead(SIG_pin);

	return val;
}

int readMux(int channel) {
	int controlPin[] = {s0, s1, s2, s3};

	int muxChannel[16][4] = {
		{0, 0, 0, 0}, // channel 0
		{1, 0, 0, 0}, // channel 1
		{0, 1, 0, 0}, // channel 2
		{1, 1, 0, 0}, // channel 3
		{0, 0, 1, 0}, // channel 4
		{1, 0, 1, 0}, // channel 5
		{0, 1, 1, 0}, // channel 6
		{1, 1, 1, 0}, // channel 7
		{0, 0, 0, 1}, // channel 8
		{1, 0, 0, 1}, // channel 9
		{0, 1, 0, 1}, // channel 10
		{1, 1, 0, 1}, // channel 11
		{0, 0, 1, 1}, // channel 12
		{1, 0, 1, 1}, // channel 13
		{0, 1, 1, 1}, // channel 14
		{1, 1, 1, 1}  // channel 15
	};

	// loop through the 4 sig
	for (int i = 0; i < 4; i++) {
		digitalWrite(controlPin[i], muxChannel[channel][i]);
	}

	int val = analogRead(SIG_pin);
	float voltage = (val * 5.0) / 1024.0;

	if (voltage < 0.1) {
		voltage = 0.0;
	}

	float percentage = (voltage / 5.0) * 100.0;
	percentage = min(percentage, 100.0);

	int roundedPercentage = static_cast<int>(round(percentage));

	return roundedPercentage;
}
