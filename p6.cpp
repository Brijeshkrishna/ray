
#include <bits/stdc++.h>

using namespace std;

int expPermute[] = {
	32, 1, 2, 3, 4, 5,
	4, 5, 6, 7, 8, 9,
	8, 9, 10, 11, 12, 13,
	12, 13, 14, 15, 16, 17,
	16, 17, 18, 19, 20, 21,
	20, 21, 22, 23, 24, 25,
	24, 25, 26, 27, 28, 29,
	28, 29, 30, 31, 32, 1};

string expansionPermute(string input)
{
	string res = "";
	for (int i = 0; i < 48; i++)
	{
		res += input[expPermute[i] - 1];
	}
	return res;
}

string XOR1(string input1, string input2)
{
	string res = "";
	for (int i = 0; i < input1.length(); i++)
	{
		res += (input1[i] == input2[i]) ? "0" : "1";
	}
	return res;
}

unsigned int sBoxes[8][4][16] = {
	{14, 4, 13, 1, 2, 15, 11, 8, 3, 10, 6, 12, 5, 9, 0, 7,
	 0, 15, 7, 4, 14, 2, 13, 1, 10, 6, 12, 11, 9, 5, 3, 8,
	 4, 1, 14, 8, 13, 6, 2, 11, 15, 12, 9, 7, 3, 10, 5, 0,
	 15, 12, 8, 2, 4, 9, 1, 7, 5, 11, 3, 14, 10, 0, 6, 13},

	{15, 1, 8, 14, 6, 11, 3, 4, 9, 7, 2, 13, 12, 0, 5, 10,
	 3, 13, 4, 7, 15, 2, 8, 14, 12, 0, 1, 10, 6, 9, 11, 5,
	 0, 14, 7, 11, 10, 4, 13, 1, 5, 8, 12, 6, 9, 3, 2, 15,
	 13, 8, 10, 1, 3, 15, 4, 2, 11, 6, 7, 12, 0, 5, 14, 9},

	{10, 0, 9, 14, 6, 3, 15, 5, 1, 13, 12, 7, 11, 4, 2, 8,
	 13, 7, 0, 9, 3, 4, 6, 10, 2, 8, 5, 14, 12, 11, 15, 1,
	 13, 6, 4, 9, 8, 15, 3, 0, 11, 1, 2, 12, 5, 10, 14, 7,
	 1, 10, 13, 0, 6, 9, 8, 7, 4, 15, 14, 3, 11, 5, 2, 12},

	{7, 13, 14, 3, 0, 6, 9, 10, 1, 2, 8, 5, 11, 12, 4, 15,
	 13, 8, 11, 5, 6, 15, 0, 3, 4, 7, 2, 12, 1, 10, 14, 9,
	 10, 6, 9, 0, 12, 11, 7, 13, 15, 1, 3, 14, 5, 2, 8, 4,
	 3, 15, 0, 6, 10, 1, 13, 8, 9, 4, 5, 11, 12, 7, 2, 14},

	{2, 12, 4, 1, 7, 10, 11, 6, 8, 5, 3, 15, 13, 0, 14, 9,
	 14, 11, 2, 12, 4, 7, 13, 1, 5, 0, 15, 10, 3, 9, 8, 6,
	 4, 2, 1, 11, 10, 13, 7, 8, 15, 9, 12, 5, 6, 3, 0, 14,
	 11, 8, 12, 7, 1, 14, 2, 13, 6, 15, 0, 9, 10, 4, 5, 3},

	{12, 1, 10, 15, 9, 2, 6, 8, 0, 13, 3, 4, 14, 7, 5, 11,
	 10, 15, 4, 2, 7, 12, 9, 5, 6, 1, 13, 14, 0, 11, 3, 8,
	 9, 14, 15, 5, 2, 8, 12, 3, 7, 0, 4, 10, 1, 13, 11, 6,
	 4, 3, 2, 12, 9, 5, 15, 10, 11, 14, 1, 7, 6, 0, 8 ,13,},

	{4, 11, 2, 14, 15, 0, 8, 13, 3, 12, 9, 7, 5, 10, 6, 1,
	 13, 0, 11, 7, 4, 9, 1, 10, 14, 3, 5, 12, 2, 15, 8, 6,
	 1, 4, 11, 13, 12, 3, 7, 14, 10, 15, 6, 8, 0, 5, 9, 2,
	 6, 11, 13, 8, 1, 4, 10, 7, 9, 5, 0, 15, 14, 2, 3, 12},

	{13, 2, 8, 4, 6, 15, 11, 1, 10, 9, 3, 14, 5, 0, 12, 7,
	 1, 15, 13, 8, 10, 3, 7, 4, 12, 5, 6, 11, 0, 14, 9, 2,
	 7, 11, 4, 1, 9, 12, 14, 2, 0, 6, 10, 13, 15, 3, 5, 8,
	 2, 1, 14, 7, 4, 10, 8, 13, 15, 12, 9, 0, 3, 5, 6, 11}};

int permTable[] = {
	16, 7, 20, 21, 29, 12, 28, 17,
	1, 15, 23, 26, 5, 18, 31, 10,
	2, 8, 24, 14, 32, 27, 3, 9,
	19, 13, 30, 6, 22, 11, 4, 25};

string substitution(string input)
{
	string res;
	for (int i = 0; i < 8; i++)
	{
		string sInput = input.substr(6 * i, 6);
		int row = bitset<2>(sInput.substr(0, 1) + sInput.substr(5, 1)).to_ulong();
		int col = bitset<4>(sInput.substr(1, 4)).to_ulong();
		res += bitset<4>(sBoxes[i][row][col]).to_string();
	}
	return res;
}

string permute(string input)
{
	string res;
	for (int i = 0; i < 32; i++)
		res += input[permTable[i] - 1];
	return res;
}

string XOR(string input1, string input2)
{
	string res;
	for (int i = 0; i < input1.length(); i++)
		res += (input1[i] == input2[i]) ? "0" : "1";
	return res;
}

int main()
{
	int i;
	unsigned long long hexInput, key;
	ifstream fin;

	cout << "Enter 64-bit Message in hex (16 digit): ";
	cin >> hex >> hexInput;

	string input = bitset<64>(hexInput).to_string();

	cout << "Enter 48-bit Round key in hex (12 digit): ";
	cin >> hex >> key;
	string Ki = bitset<48>(key).to_string();

	cout << "\n64-bit Binary Input = " << input << endl;
	cout << "key for ith round (Ki) = " << Ki << endl;

	string Ri_1 = input.substr(32, 32);
	cout << "\nRight half of 64-bit input = " << Ri_1 << " : 0x" << hex << bitset<32>(Ri_1).to_ulong() << endl;

	string R48 = expansionPermute(Ri_1);
	cout << "Right half after expansion permutation = " << R48 << " : 0x" << hex << bitset<48>(R48).to_ulong() << endl;

	string sBoxInput = XOR1(R48, Ki);

	cout << "\nAfter XOR of Right half and round key : " << sBoxInput << " : " << hex << bitset<48>(sBoxInput).to_ulong() << endl;
	unsigned long long int a = bitset<48>(sBoxInput).to_ulong();

	string sBoxinput = bitset<48>(a).to_string();

	string Li_1 = input.substr(0, 32);
	cout << "Right half of 64-bit input = : " << Li_1 << " : 0x" << hex << bitset<32>(Li_1).to_ulong() << endl;

	string sBoxOutput = substitution(sBoxinput);
	cout << "\nS-Box output	= " << sBoxOutput << " : 0x" << hex << bitset<32>(sBoxInput).to_ulong() << endl;

	string P = permute(sBoxOutput);
	cout << "Permuted output = " << P << " : " << hex << bitset<32>(P).to_ulong() << endl;

	string Ri = XOR(P, Li_1);
	cout << "\nOutput of round = " << Ri << " : 0x" << hex << bitset<32>(Ri).to_ulong() << endl;

	return 0;
}
