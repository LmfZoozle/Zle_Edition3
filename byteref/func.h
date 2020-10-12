#include <fstream>
#include <iostream>

using namespace std;
void showall_code(vector<string>& code){
    cout<<"show all bytecodes and mnemonics"<<endl<<endl;
    for(auto &run:code){
        cout<<run<<endl;
    }
    cout<<endl;
}