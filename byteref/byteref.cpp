#include <string>
#include <iostream>
#include <algorithm>
#include <fstream>
#include <vector>
#include "func.h"

using namespace std;

const string path = "bytecode.txt";

//バイトコードから検索するのと
//ニーモニックから検索するのと
//全部晒すのと

class showall_exception:public std::exception{
    public:
    string what;
    showall_exception(string wh){
        what=wh;
    }
};

int main(int argc,char** argv)
{
    cout << "bytecode reference and search." << endl;
    ifstream catalog(path);
    if (!catalog)
    {
        cout << "Cannot find \"bytecode.txt\"" << endl;
        cout << "Aborting to due previous problem..." << endl;
    }
    vector<string> lines;

    //ここはいらない行を捨ててるだけ
    string tmpa;
    getline(catalog,tmpa);
    getline(catalog,tmpa);

    while (!catalog.eof())
    {
        string tmp;
        getline(catalog, tmp);
        lines.emplace_back(tmp);
        //tmp.clear();
        //cout << lines.back() << endl;
    }

    while (1)
    {
        string input;
        bool is_num=true;
        cout << "Enter the bytecode or mnemonic." << endl;
        cout<<"Also you can see all of them by \"showall\" "<<endl;
        getline(cin, input);

        //cout << endl<< input << endl;
        try
        {
            if (input=="showall"){
                throw showall_exception(input);
            }
            int code = stoi(input, nullptr, 10);
            if (code>11111111){
                cout<<"Argument is too big."<<endl;
            }
            if (code<0){
                cout<<"Argument is too small."<<endl;

            }
        //    cout << "Number -> bytecode" << endl;
        }
        catch (showall_exception){
            showall_code(lines);
        }
        catch (std::invalid_argument)
        {
        }
    }

    return 0;
}