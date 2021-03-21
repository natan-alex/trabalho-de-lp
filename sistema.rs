// a palavra chave use é responsável por trazer ao
// escopo atual módulos que contenham funcionalidades 
// externas ao programa. Se assemelha ao import do 
// Java e o include do C

// o operador :: (dois pontos duas vezes) é utilizado para
// fazer acesso a métodos ou funções definidas em uma struct, enum,
// traits, etc (são tipos de dados compostos do rust) ou até mesmo
// a funções declaradas em módulos externos ou a outros módulos
use std::io;
use std::cmp::PartialOrd;

// fn é a notação utilizada para declarar funções:
// Rust possui o escopo léxico, que utiliza de blocos, que,
// no caso dessa linguagem são delimitados por chaves, para agrupar o corpo de uma função
// ou delimitar todo um escopo, portanto o corpo da função é envolto
// por chaves
fn main() {
    // println! é uma macro, estrutura que se assemelha a uma função
    println!("Escolha o tipo de dados dos elementos que serão ordenados: ");
    println!("[1] - string");
    println!("[2] - int");
    println!("[3] - float");
    println!("Opção: ");

    // o let é utilizado para declarar novas variáveis
    // mut é para tornar uma variável mutável, isso pois,
    // por padrão, uma variável é imutável em Rust, ou seja,
    // não pode ter seu conteúdo alterado
    let mut opcao = String::new();

    // o operador . (ponto final) é utilizado para acessar campos
    // de um struct ou encadear funções ou métodos a partir do retorno
    // de um método anterior
    io::stdin()
        .read_line(&mut opcao) // ler dados da entrada padrão e associar a variável opcao
        .expect("Falha ao ler o tipo dos dados!"); // em caso de falha, a
    // função expect é responsável por causar o fim do programa, o crash, sendo
    // o motivo do erro indicado como parâmetro

    // a notação <> é utilizada para especificar o tipo que será 
    // o tipo do parâmetro a ser passado a uma função que lida com
    // generics. Os generics, como o nome diz, estão relacionados a funções 
    // ou métodos que podem receber parâmetros de tipos genéricos, sem ter
    // de se limitar a um tipo específico de parâmetros. São semelhantes aos
    // generics da linguagem Java, por exemplo.
    let opcao = opcao
        .trim() // remover espaços no começo e no fim de uma string
        .parse::<i32>() // converter o valor de uma string para o tipo indicado nos <>
        .expect("Falha ao ler o tipo dos dados: opção inválida.");

    // o operador match é responsável por identificar, dentre as opções
    // explicitadas, qual o pattern(padrão) que "casa" com determinada variável.
    // É usado no contexto de casamento de padrões. Dentre uma das possibilidades,
    // com qual dos padrões a variável se encaixa? Com base nisso, podemos seguir
    // caminhos diferentes e fazer operações diferentes.
    let opcao = match opcao {
        // 0..=4 é dito um range, ou seja, um conjunto de números inteiros que 
        // pertencem ao segmento que vai de um limite inferior até um limite superior

        // à direita da => está o código que será executado caso a variável que foi
        // submetida ao match "case" com o padrão dito na esquerda, que nesse caso,
        // como é um range, poderia ser um dos números 0, 1, 2, 3 ou o 4.
        0..=4 => opcao,
        // o _ significa qualquer coisa e "casa" com qualquer valor que a variável
        // possui. Como o match lida com exaustão, ou seja, necessita que todos os 
        // valores possíveis, todos os padrões possíveis sejam explicitados, com seus respectivos códigos,
        // o uso de _ evita que tenhamos que escrever todos os padrões e, os que não foram
        // identificados antes do _ caiam no caso do próprio _.
        _ => {
            println!("Opção inválida! Por padrão o tipo será número inteiro...");
            2 // um valor ou toda uma lógica que não termine com ; é dito uma expressão
            // e tem o valor de seu resultado sempre retornado ao código. Como 2
            // nesse caso não termina com ; , seu valor é retornado como resultado
            // do código que caia no padrão _, ou seja, qualquer valor possível
            // que não tenha sido identificado anteriormente no bloco do match
        }
    };
    // o valor retornado do bloco match é associado à variável opcao e, por esse
    // motivo, como essa é considerada uma declaração de variável, o bloco é finalizado
    // por ;

    let mut dados_digitados = String::new();

    println!("Digite os dados a serem ordenados (em sequência e separados por espaços): ");
    io::stdin()
        .read_line(&mut dados_digitados)
        .expect("Falha ao ler dados.");


    // o operador & é utilizado para lidar com referências a valores que são armazenados
    // na estrutura heap da memória, ditos dinâmicos. Posteriormente, a expressão delimitada
    // pelos [] identifica que os dados contidos na variável entre o & e os colchetes
    // será particionado, chamado de slice, e, essa partição, que possui um range expecificado
    // dentro dos colchetes, terá seu valor retornado à variável presente na declaração.

    // o uso do & indica que ocorrerá um borrow do valor de uma variável, o que significa que
    // momentaneamente, os dados dessa variável serão acessados por outra, informalmente serão
    // emprestados a variável da declaração
    let dados_digitados = &dados_digitados[0..dados_digitados.len()-1]; // remover o caractere \n do fim da linha por meio do slice de string:

    // aqui, o : logo após o nome da variável indica
    // que o tipo da variável será indicado logo a seguir
    let mut vetor: Vec<&str> = dados_digitados.trim().split(" ").collect();
    // a função split quebra uma string em valores separados a partir do delimitador
    // passado como parâmetro e a função collect agrupa esses valores separados em 
    // um vetor e o retorna

    let mut vetor_inteiros = Vec::<i32>::new();
    let mut vetor_floats = Vec::<f64>::new();

    // o operador de controle de fluxo if determina uma tomada de decisão baseada
    // em uma condição que, caso seja satisfeita, ou seja, tenha um valor verdadeiro,
    // irá possibilitar a execução de um bloco de código delimitado pelo bloco do if
    if opcao == 2 {
        // o operador for é utilizado para executar um bloco de código
        // de maneira repetitiva e percorrer estruturas que sejam iteráveis
        for item in vetor {
            vetor_inteiros.push(item.trim().parse::<i32>().unwrap());
            // um vetor nativamente possui tamanho dinâmico no Rust, de forma que podemos adicionar
            // novos valores com a função push. A função unwrap também é responsável por causar o
            // crash do programa porém não possibilita explicitar o erro de maneira personalizada
            // como a função except
        }
        heapsort(&mut vetor_inteiros);
        println!("Vetor ordenado:");
        mostrar_itens_do_vetor(&vetor_inteiros);
    } else if opcao == 3 {
        for item in vetor {
            vetor_floats.push(item.trim().parse::<f64>().unwrap());
        }
        heapsort(&mut vetor_floats);
        println!("Vetor ordenado:");
        mostrar_itens_do_vetor(&vetor_floats);
    } else {
        heapsort(&mut vetor);
        println!("Vetor ordenado:");
        mostrar_itens_do_vetor(&vetor);
    }
}

// o uso de <> logo após o nome da função indica o uso de generics por parte
// da função. O operador : , nesse caso, indica que o tipo que será usado pelo parâmetro da 
// função deve implementar uma certa trait, que é como se fosse uma interface
// em linguagens orientadas a objetos como o Java. Além disso, mais a frente na
// declaração, a sequência &[T] indica que o parâmetro nomeado de vetor terá
// o tipo genérico T e será um vetor que terá seus dados emprestados à função
// momentaneamente, o que representa um borrow dos dados do vetor
fn mostrar_itens_do_vetor<T: std::fmt::Display> (vetor: &[T]) {
    print!("[ ");
    for item in vetor {
        print!("{} ", item);
    }
    println!("]");
}

// o uso de &mut ... indica que o borrow dessa vez será de uma referência mutável,
// ou seja, não será mais um borrow, mas sim uma transferẽncia momentânea de dono
// dos dados que estavam associados ao vetor, de forma que os dados podem sofrer
// modificações. Esse conceito de dono é chamado de ownership na linguagem, sendo
// que somente um dono por vez pode ser capaz de mudar os determinado conjunto de
// dados em um escopo.

// o tipo T deve implementar as traits Copy e PartialOrd para que possam ser comparáveis com
// os operadores <, >, etc e possíveis de serem copiados para outra variável sem que tenham
// seu ownership transferidos
fn heapsort<T: PartialOrd + Copy> (vetor: &mut [T]) {
    let tamanho_do_vetor = vetor.len();
    let mut i = (tamanho_do_vetor-1)/2;
    // while é um outro operador utilizado para
    // realizar repetições de um determinado bloco
    // de código enquanto a condição for verdadeira
    while i > 0 {
        heapify(vetor, i, tamanho_do_vetor);
        // o operador -= é um facilitador para a operação
        // que diz que o valor atual do número será subtraído
        // do número que vier a seguir. Nesse caso seria
        // o mesmo que dizer i = i - 1;
        i -= 1;
    }

    heapify(vetor, 0, tamanho_do_vetor);

    i = tamanho_do_vetor-1;
    while i > 0 {
        // no caso de um vetor, para acessar um certo valor
        // que está localizado em determinado índice do vetor,
        // esse valor pode ser acessado utilizado o nome do vetor
        // seguido de [] com o índice correspondente dentro desses colchetes
        let aux = vetor[0];
        vetor[0] = vetor[i];
        vetor[i] = aux;
        heapify(vetor, 0, i);
        i -= 1;
    }
}

fn heapify<T: PartialOrd + Copy> (vetor: &mut [T], pos_do_pai: usize, tamanho_do_vetor: usize) {
    let pos_filho1 = pos_do_pai*2+1; 
    let pos_filho2 = pos_do_pai*2+2; 
    let mut pos_maior_filho = pos_do_pai;

    // o uso de && indica uma operação lógica AND
    if pos_filho1 < tamanho_do_vetor && vetor[pos_filho1] > vetor[pos_maior_filho] {
        pos_maior_filho = pos_filho1;
    }

    if pos_filho2 < tamanho_do_vetor && vetor[pos_filho2] > vetor[pos_maior_filho] {
        pos_maior_filho = pos_filho2;
    }

    if pos_maior_filho != pos_do_pai {
        let aux = vetor[pos_maior_filho];
        vetor[pos_maior_filho] = vetor[pos_do_pai];
        vetor[pos_do_pai] = aux;

        heapify(vetor, pos_maior_filho, tamanho_do_vetor);
    }
}
