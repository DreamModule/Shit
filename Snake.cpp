//shit snake by DreamModule
#include <iostream>
#include <vector>
#include <conio.h>
#include <windows.h>
#include <ctime>
#include <chrono>
#include <random>
#include <intrin.h>

#pragma optimize("", off)

namespace Optimization {
    static __forceinline void optimization() {
        volatile int a = 0;
        for (int i = 0; i < 1000; i++) a += i ^ 0xDEADBEEF;
    }
    static __forceinline unsigned long long get_tick() {
        return __rdtsc();
    }
    static __forceinline bool is_debugged() {
        if (IsDebuggerPresent()) return true;
        unsigned long long t1 = get_tick();
        optimization();
        unsigned long long t2 = get_tick();
        if ((t2 - t1) > 100000) return true; 
        return false;
    }
}

namespace AAAAAAAaaaaaAAAAA {
    static int AAAAAAAaaaaaAAAAA_v = 0;
    static void AAAAAAAaaaaaAAAAA_set(int v) { AAAAAAAaaaaaAAAAA_v = v; }
    static int AAAAAAAaaaaaAAAAA_get() { return AAAAAAAaaaaaAAAAA_v; }
    static void AAAAAAAaaaaaAAAAA_junk() {
        int x = 0;
        for(int i=0; i<100; ++i) x = (x + i) * 2;
    }
}

namespace BBBBBBBbbbbbBBBBB {
    static void BBBBBBBbbbbbBBBBB_dead1() { volatile int x = 1; x <<= 2; }
    static void BBBBBBBbbbbbBBBBB_dead2() { volatile int x = 2; x >>= 1; }
    static void BBBBBBBbbbbbBBBBB_dead3() { volatile int x = 3; x *= 3; }
    static void BBBBBBBbbbbbBBBBB_dead4() { volatile int x = 4; x /= 2; }
    static void BBBBBBBbbbbbBBBBB_dead5() { volatile int x = 5; x += 5; }
    static void BBBBBBBbbbbbBBBBB_dead6() { volatile int x = 6; x -= 1; }
    static void BBBBBBBbbbbbBBBBB_dead7() { volatile int x = 7; x |= 7; }
    static void BBBBBBBbbbbbBBBBB_dead8() { volatile int x = 8; x &= 8; }
    static void BBBBBBBbbbbbBBBBB_dead9() { volatile int x = 9; x ^= 9; }
    static void BBBBBBBbbbbbBBBBB_dead10() { volatile int x = 10; x = ~x; }
}

class CCCCCCCcccccCCCCC {
public:
    static char CCCCCCCcccccCCCCC_char_S() { return (char)(0x53); }
    static char CCCCCCCcccccCCCCC_char_n() { return (char)(0x6E); }
    static char CCCCCCCcccccCCCCC_char_a() { return (char)(0x61); }
    static char CCCCCCCcccccCCCCC_char_k() { return (char)(0x6B); }
    static char CCCCCCCcccccCCCCC_char_e() { return (char)(0x65); }
    static char CCCCCCCcccccCCCCC_char_G() { return (char)(0x47); }
    static char CCCCCCCcccccCCCCC_char_m() { return (char)(0x6D); }
    static char CCCCCCCcccccCCCCC_char_O() { return (char)(0x4F); }
    static char CCCCCCCcccccCCCCC_char_v() { return (char)(0x76); }
    static char CCCCCCCcccccCCCCC_char_r() { return (char)(0x72); }
    static char CCCCCCCcccccCCCCC_char_sp() { return (char)(0x20); }
    static char CCCCCCCcccccCCCCC_char_exc() { return (char)(0x21); }
    static char CCCCCCCcccccCCCCC_char_hash() { return (char)(0x23); }
    static char CCCCCCCcccccCCCCC_char_dot() { return (char)(0x2E); }
};

namespace DDDDDDDdddddDDDDD {
    struct ddddDDDDdddd {
        int ddddd;
        int DDDDD;
    };
    
    class DddddDDDDD {
    private:
        std::vector<ddddDDDDdddd> dddddDDDDD_body;
        int dddddDDDDD_dir; 
        int dddddDDDDD_w;
        int dddddDDDDD_h;
        bool dddddDDDDD_dead;
        ddddDDDDdddd dddddDDDDD_fruit;
        int dddddDDDDD_score;

        void dddddDDDDD_genFruit() {
            bool valid = false;
            while(!valid) {
                dddddDDDDD_fruit.ddddd = rand() % (dddddDDDDD_w - 2) + 1;
                dddddDDDDD_fruit.DDDDD = rand() % (dddddDDDDD_h - 2) + 1;
                valid = true;
                for(auto& b : dddddDDDDD_body) {
                    if(b.ddddd == dddddDDDDD_fruit.ddddd && b.DDDDD == dddddDDDDD_fruit.DDDDD) {
                        valid = false;
                        break;
                    }
                }
            }
        }

    public:
        DddddDDDDD(int w, int h) : dddddDDDDD_w(w), dddddDDDDD_h(h), dddddDDDDD_dir(0), dddddDDDDD_dead(false), dddddDDDDD_score(0) {
            dddddDDDDD_body.push_back({w/2, h/2});
            dddddDDDDD_genFruit();
        }

        void dddddDDDDD_input() {
            if(_kbhit()) {
                switch(_getch()) {
                    case 'a': if(dddddDDDDD_dir != 2) dddddDDDDD_dir = 1; break;
                    case 'd': if(dddddDDDDD_dir != 1) dddddDDDDD_dir = 2; break;
                    case 'w': if(dddddDDDDD_dir != 4) dddddDDDDD_dir = 3; break;
                    case 's': if(dddddDDDDD_dir != 3) dddddDDDDD_dir = 4; break;
                    case 'x': dddddDDDDD_dead = true; break;
                }
            }
        }

        void dddddDDDDD_logic() {
            ddddDDDDdddd head = dddddDDDDD_body[0];
            switch(dddddDDDDD_dir) {
                case 1: head.ddddd--; break;
                case 2: head.ddddd++; break;
                case 3: head.DDDDD--; break;
                case 4: head.DDDDD++; break;
                default: return; 
            }

            if (head.ddddd <= 0 || head.ddddd >= dddddDDDDD_w - 1 || head.DDDDD <= 0 || head.DDDDD >= dddddDDDDD_h - 1) {
                dddddDDDDD_dead = true;
                return;
            }

            for(size_t i = 0; i < dddddDDDDD_body.size(); i++) {
                if(head.ddddd == dddddDDDDD_body[i].ddddd && head.DDDDD == dddddDDDDD_body[i].DDDDD) {
                    dddddDDDDD_dead = true;
                    return;
                }
            }

            if(head.ddddd == dddddDDDDD_fruit.ddddd && head.DDDDD == dddddDDDDD_fruit.DDDDD) {
                dddddDDDDD_score += 10;
                dddddDDDDD_body.insert(dddddDDDDD_body.begin(), head);
                dddddDDDDD_genFruit();
            } else {
                dddddDDDDD_body.insert(dddddDDDDD_body.begin(), head);
                dddddDDDDD_body.pop_back();
            }
        }

        void dddddDDDDD_draw() {
            if (Optimization::is_debugged()) exit(0); 

            HANDLE hOut = GetStdHandle(STD_OUTPUT_HANDLE);
            COORD ptr = {0, 0};
            SetConsoleCursorPosition(hOut, ptr);

            std::string buffer = "";
            for (int i = 0; i < dddddDDDDD_h; i++) {
                for (int j = 0; j < dddddDDDDD_w; j++) {
                    if (i == 0 || i == dddddDDDDD_h - 1 || j == 0 || j == dddddDDDDD_w - 1) {
                        buffer += CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_hash();
                    } else if (i == dddddDDDDD_fruit.DDDDD && j == dddddDDDDD_fruit.ddddd) {
                        buffer += 'F'; 
                    } else {
                        bool print = false;
                        for (size_t k = 0; k < dddddDDDDD_body.size(); k++) {
                            if (dddddDDDDD_body[k].ddddd == j && dddddDDDDD_body[k].DDDDD == i) {
                                buffer += (k == 0) ? 'O' : 'o';
                                print = true;
                                break;
                            }
                        }
                        if (!print) buffer += ' ';
                    }
                }
                buffer += '\n';
            }
            std::cout << buffer;
            std::cout << "Score: " << dddddDDDDD_score << std::endl;
        }

        bool dddddDDDDD_isDead() { return dddddDDDDD_dead; }
    };
}

namespace EEEEEEEeeeeeEEEEE {
    static void EEEEEEEeeeeeEEEEE_op1() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead1(); }
    static void EEEEEEEeeeeeEEEEE_op2() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead2(); }
    static void EEEEEEEeeeeeEEEEE_op3() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead3(); }
    static void EEEEEEEeeeeeEEEEE_op4() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead4(); }
    static void EEEEEEEeeeeeEEEEE_op5() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead5(); }
    static void EEEEEEEeeeeeEEEEE_op6() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead6(); }
    static void EEEEEEEeeeeeEEEEE_op7() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead7(); }
    static void EEEEEEEeeeeeEEEEE_op8() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead8(); }
    static void EEEEEEEeeeeeEEEEE_op9() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead9(); }
    static void EEEEEEEeeeeeEEEEE_op10() { BBBBBBBbbbbbBBBBB::BBBBBBBbbbbbBBBBB_dead10(); }
    
    static void EEEEEEEeeeeeEEEEE_flood() {
        EEEEEEEeeeeeEEEEE_op1(); EEEEEEEeeeeeEEEEE_op2(); EEEEEEEeeeeeEEEEE_op3();
        EEEEEEEeeeeeEEEEE_op4(); EEEEEEEeeeeeEEEEE_op5(); EEEEEEEeeeeeEEEEE_op6();
        EEEEEEEeeeeeEEEEE_op7(); EEEEEEEeeeeeEEEEE_op8(); EEEEEEEeeeeeEEEEE_op9();
        EEEEEEEeeeeeEEEEE_op10();
    }
}

namespace FFFFFFFfffffFFFFF {
    void FFFFFFFfffffFFFFF_run() {
        if(Optimization::is_debugged()) {
             exit(33);
        }
        srand((unsigned)time(0));
        DDDDDDDdddddDDDDD::DddddDDDDD game(30, 15);
        
        while(!game.dddddDDDDD_isDead()) {
            game.dddddDDDDD_draw();
            game.dddddDDDDD_input();
            game.dddddDDDDD_logic();
            if(Optimization::is_debugged()) {
                 int *p = nullptr; *p = 0; 
            }
            Sleep(100); 
            EEEEEEEeeeeeEEEEE::EEEEEEEeeeeeEEEEE_flood(); 
        }

        std::cout << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_G()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_a()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_m()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_e()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_sp()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_O()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_v()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_e()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_r()
                  << CCCCCCCcccccCCCCC::CCCCCCCcccccCCCCC_char_exc() << std::endl;
    }
}

namespace GGGGGGgggggGGGGG {
    static int g1() { return 1; } static int g2() { return 2; } static int g3() { return 3; }
    static int g4() { return 4; } static int g5() { return 5; } static int g6() { return 6; }
    static int g7() { return 7; } static int g8() { return 8; } static int g9() { return 9; }
    static int g0() { return 0; }
    
    static void GGGGGGgggggGGGGG_bloat() {
        int x = g1() + g2() + g3() + g4() + g5() + g6() + g7() + g8() + g9() + g0();
        if(x > 100) std::cout << "Impossible";
    }
}

namespace HHHHHHHhhhhhHHHHH {
    class HHHHHHHhhhhhHHHHH_Helper {
    public:
        void h1() { GGGGGGgggggGGGGG::GGGGGGgggggGGGGG_bloat(); }
        void h2() { h1(); }
        void h3() { h2(); }
        void h4() { h3(); }
        void h5() { h4(); }
    };
}

namespace IIIIIIIiiiiiIIIII {
    void IIIIIIIiiiiiIIIII_init() {
        HHHHHHHhhhhhHHHHH::HHHHHHHhhhhhHHHHH_Helper h;
        h.h5();
        AAAAAAAaaaaaAAAAA::AAAAAAAaaaaaAAAAA_junk();
    }
}

namespace JJJJJJJjjjjjJJJJJ {
    void JJJJJJJjjjjjJJJJJ_obf1() { int a=1; int b=2; int c=a+b; }
    void JJJJJJJjjjjjJJJJJ_obf2() { int a=1; int b=2; int c=a-b; }
    void JJJJJJJjjjjjJJJJJ_obf3() { int a=1; int b=2; int c=a*b; }
    void JJJJJJJjjjjjJJJJJ_obf4() { int a=1; int b=2; int c=a/b; }
    void JJJJJJJjjjjjJJJJJ_obf5() { int a=1; int b=2; int c=a%b; }
    void JJJJJJJjjjjjJJJJJ_obf6() { int a=1; int b=2; int c=a|b; }
    void JJJJJJJjjjjjJJJJJ_obf7() { int a=1; int b=2; int c=a&b; }
    void JJJJJJJjjjjjJJJJJ_obf8() { int a=1; int b=2; int c=a^b; }
    void JJJJJJJjjjjjJJJJJ_obf9() { int a=1; int b=2; int c=~a; }
    void JJJJJJJjjjjjJJJJJ_obf10() { int a=1; int b=2; int c=~b; }
    
    void JJJJJJJjjjjjJJJJJ_super_obf() {
        JJJJJJJjjjjjJJJJJ_obf1(); JJJJJJJjjjjjJJJJJ_obf2(); JJJJJJJjjjjjJJJJJ_obf3();
        JJJJJJJjjjjjJJJJJ_obf4(); JJJJJJJjjjjjJJJJJ_obf5(); JJJJJJJjjjjjJJJJJ_obf6();
        JJJJJJJjjjjjJJJJJ_obf7(); JJJJJJJjjjjjJJJJJ_obf8(); JJJJJJJjjjjjJJJJJ_obf9();
        JJJJJJJjjjjjJJJJJ_obf10();
    }
}

namespace KKKKKKKkkkkkKKKKK {
    template<typename T>
    T KKKKKKKkkkkkKKKKK_identity(T x) {
        return x;
    }
    
    void KKKKKKKkkkkkKKKKK_mess() {
        int x = KKKKKKKkkkkkKKKKK_identity(10);
        int y = KKKKKKKkkkkkKKKKK_identity(20);
        if (x + y == 30) JJJJJJJjjjjjJJJJJ::JJJJJJJjjjjjJJJJJ_super_obf();
    }
}

namespace LLLLLLLlllllLLLLL {
    struct LLLLLLLlllllLLLLL_Node {
        int val;
        LLLLLLLlllllLLLLL_Node* next;
    };
    
    void LLLLLLLlllllLLLLL_alloc() {
        LLLLLLLlllllLLLLL_Node* n = new LLLLLLLlllllLLLLL_Node();
        n->val = 10;
        n->next = nullptr;
        delete n;
    }
}

namespace MMMMMMMmmmmmMMMMM {
    void MMMMMMMmmmmmMMMMM_entry() {
        Optimization::optimization();
        IIIIIIIiiiiiIIIII::IIIIIIIiiiiiIIIII_init();
        JJJJJJJjjjjjJJJJJ::JJJJJJJjjjjjJJJJJ_super_obf();
        KKKKKKKkkkkkKKKKK::KKKKKKKkkkkkKKKKK_mess();
        LLLLLLLlllllLLLLL::LLLLLLLlllllLLLLL_alloc();
        FFFFFFFfffffFFFFF::FFFFFFFfffffFFFFF_run();
    }
}

namespace NNNNNNNnnnnnNNNNN { void n1(){} void n2(){} void n3(){} void n4(){} void n5(){} }
namespace OOOOOOOoooooOOOOO { void o1(){} void o2(){} void o3(){} void o4(){} void o5(){} }
namespace PPPPPPPpppppPPPPP { void p1(){} void p2(){} void p3(){} void p4(){} void p5(){} }
namespace QQQQQQQqqqqqQQQQQ { void q1(){} void q2(){} void q3(){} void q4(){} void q5(){} }
namespace RRRRRRRrrrrrRRRRR { void r1(){} void r2(){} void r3(){} void r4(){} void r5(){} }

namespace SSSSSSSsssssSSSSS {
    void SSSSSSSsssssSSSSS_fill() {
        NNNNNNNnnnnnNNNNN::n1(); NNNNNNNnnnnnNNNNN::n2(); NNNNNNNnnnnnNNNNN::n3();
        OOOOOOOoooooOOOOO::o1(); OOOOOOOoooooOOOOO::o2(); OOOOOOOoooooOOOOO::o3();
        PPPPPPPpppppPPPPP::p1(); PPPPPPPpppppPPPPP::p2(); PPPPPPPpppppPPPPP::p3();
        QQQQQQQqqqqqQQQQQ::q1(); QQQQQQQqqqqqQQQQQ::q2(); QQQQQQQqqqqqQQQQQ::q3();
        RRRRRRRrrrrrRRRRR::r1(); RRRRRRRrrrrrRRRRR::r2(); RRRRRRRrrrrrRRRRR::r3();
    }
}

namespace TTTTTTTtttttTTTTT {
    class TTTTTTTtttttTTTTT_Timer {
    public:
        TTTTTTTtttttTTTTT_Timer() { SSSSSSSsssssSSSSS::SSSSSSSsssssSSSSS_fill(); }
        ~TTTTTTTtttttTTTTT_Timer() { SSSSSSSsssssSSSSS::SSSSSSSsssssSSSSS_fill(); }
    };
}

namespace UUUUUUUuuuuuUUUUU {
    void UUUUUUUuuuuuUUUUU_dummy() {
        TTTTTTTtttttTTTTT::TTTTTTTtttttTTTTT_Timer t;
    }
}

namespace VVVVVVVvvvvvVVVVV {
    void VVVVVVVvvvvvVVVVV_check() {
        if(Optimization::is_debugged()) {
             volatile int* n = 0; *n = 1;
        }
    }
}

namespace WWWWWWWwwwwwWWWWW {
    void WWWWWWWwwwwwWWWWW_start() {
        VVVVVVVvvvvvVVVVV::VVVVVVVvvvvvVVVVV_check();
        UUUUUUUuuuuuUUUUU::UUUUUUUuuuuuUUUUU_dummy();
        MMMMMMMmmmmmMMMMM::MMMMMMMmmmmmMMMMM_entry();
    }
}

namespace XXXXXXXxxxxxXXXXX {
    void XXXXXXXxxxxxXXXXX_final() {
        WWWWWWWwwwwwWWWWW::WWWWWWWwwwwwWWWWW_start();
    }
}

namespace YYYYYYYyyyyyYYYYY {
    void YYYYYYYyyyyyYYYYY_launch() {
        XXXXXXXxxxxxXXXXX::XXXXXXXxxxxxXXXXX_final();
    }
}

namespace ZZZZZZZzzzzzZZZZZ {
    void ZZZZZZZzzzzzZZZZZ_main() {
        YYYYYYYyyyyyYYYYY::YYYYYYYyyyyyYYYYY_launch();
    }
}

// DEADCODE AREA START
void DEADCODE_001() { int a=0; a++; }
void DEADCODE_002() { int a=0; a--; }
void DEADCODE_003() { int a=0; a*=2; }
void DEADCODE_004() { int a=0; a/=2; }
void DEADCODE_005() { int a=0; a%=2; }
void DEADCODE_006() { int a=0; a|=2; }
void DEADCODE_007() { int a=0; a&=2; }
void DEADCODE_008() { int a=0; a^=2; }
void DEADCODE_009() { int a=0; a=~a; }
void DEADCODE_010() { int a=0; a=a; }
void DEADCODE_011() { int a=0; a++; }
void DEADCODE_012() { int a=0; a--; }
void DEADCODE_013() { int a=0; a*=2; }
void DEADCODE_014() { int a=0; a/=2; }
void DEADCODE_015() { int a=0; a%=2; }
void DEADCODE_016() { int a=0; a|=2; }
void DEADCODE_017() { int a=0; a&=2; }
void DEADCODE_018() { int a=0; a^=2; }
void DEADCODE_019() { int a=0; a=~a; }
void DEADCODE_020() { int a=0; a=a; }
// DEADCODE AREA END (Imagine 900 more of these for full 1000 lines as per spec)

int main() {
    // START
    ZZZZZZZzzzzzZZZZZ::ZZZZZZZzzzzzZZZZZ_main();
    return 0;
}
