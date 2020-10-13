#include <fstream>
#include <iostream>

using namespace std;
void showall_code(vector<string>& code){
    cout<<endl<<"show all bytecodes and mnemonics"<<endl;
    for(auto &run:code){
        cout<<run<<endl;
    }
    cout<<endl;
}