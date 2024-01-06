#include <Wire.h>
#include <LiquidCrystal_I2C.h>

const int DELAY = 75;
const int POTS = 4;
int potValues[] = {0, 0, 0, 0};
int potIds[] = {A0, A1, A2, A3};

LiquidCrystal_I2C lcd(0x27, 16, 2);

void setup() {
  Serial.begin(115200);
  lcd.begin(16, 2);
}

void loop() {
  for (int i = 0; i < POTS; i++) {
    int analogValue = analogRead(potIds[i]);
    // Calculate the value from 0-100 based on the analog value (0-1023)
    int value = map(analogValue, 0, 1023, 0, 100);
    if (value == 99) {
      value = 100;
    }
    potValues[i] = value;
    Serial.print("pot");
    Serial.print(i);
    Serial.print("=");
    Serial.println(value);
  }

  lcd.clear();

  lcd.setCursor(0, 0);
  lcd.print("MIC:");
  lcd.print(potValues[0] < 10 ? "  " : potValues[0] < 100 ? " " : ""); // Pad with spaces
  lcd.print(potValues[0]);
  lcd.print("|SPT:");
  lcd.print(potValues[1] < 10 ? "  " : potValues[1] < 100 ? " " : ""); // Pad with spaces
  lcd.print(potValues[1]);

  lcd.setCursor(0, 1);
  lcd.print("DSC:");
  lcd.print(potValues[2] < 10 ? "  " : potValues[2] < 100 ? " " : ""); // Pad with spaces
  lcd.print(potValues[2]);
  lcd.print("|GME:");
  lcd.print(potValues[3] < 10 ? "  " : potValues[3] < 100 ? " " : ""); // Pad with spaces
  lcd.print(potValues[3]);

  delay(DELAY);
}
