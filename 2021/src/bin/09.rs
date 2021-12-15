use std::collections::VecDeque;

use itertools::Itertools;

static INPUT: &str = "5456789349886456890123985435578996543213456789656899996467789234989765442345789778999989652349879899\n4349891298765348789339875323456789665434568996545698874356679959879898321457893569998879931998765668\n1298910989873234595498764312345678976746899989656987563234567899767987442578954678987968899897654457\n2987939875432123489999953201234599698657979979997965432023479998959876553689965789876856789789543345\n9896899984321012668899865313546789569798965469879876553135568987643988767997896898765945697698722256\n8765789965442143456789996579658895434999876398767987864589679876542099898966789999833123598589810123\n9954629876553234667899987988767932129899989219755398878678989987943989959355678998921057987678924345\n6543212989654345788999898999998941098789998998543229989789797999899876543134989997632345698789545456\n8654543498765476899398759126679953997678987987632101297997656798678965432015699876546559989898756567\n8767654579879989943297642014567899889567896597543223456789348976578976543126921998758698979999768979\n9988765678998999765965432123789998765456997498654345789893219865458897654434890139769987867896979989\n9899876989987899899876843234891249654369889329765567899994325976346789887565789349898765756975395699\n8767987899876999997987755346910198773235679939876688989875634987497893999679899959999654248954234799\n9653299999875789986599896467899987652134567899999789879876786798998932398789999898998643135932129978\n8632102398764578965434987578978698743028979989998998866987899899989510129999987797987659239891098769\n6543223987643589874323498678965469764167899878987897654398999989876431236789876576898798998789989756\n7654564597532599993214998789975379894356899865476789762129789879876543345699765455789987684569877546\n8799796798543489874109899899876123989456989874345678995345698765987876596998954314597998593498766434\n9989987987654678965929799999993239979569876543234589986797987653298989989897895423456987432987655325\n9876898998779899899898589998754998768998765432107678999989298942139599879766889435668986543498943212\n9785799659989956798767467899769878657889877543234789998979109893013498765455679546899598665569954323\n9654588949893245697654356999898767345678987698645679876767998782129987654343568958912459988798766534\n7543567998732196899793234789999543236789699798759798765656897643298799973212389979101967999899887646\n6432457898653989932989345699987684545893568999868979874545989654349698764324567899999899767999998787\n7421348999769878949878996789999876657932378999979657953435679876598539875634689979876798657789429898\n3210128799898767956965989899987988867891234989989345942124789998987621986875678967965987545679945929\n4323235678923456899854567978976799978910349878991249873675678989798710197987989459893296534567896912\n5454348789636567989765878967965456989321398767890956954578789977679821298998994299794987321457999899\n7689656896547678979896989549876567895432999656799897967789897855569932459019865987689876432346899768\n9798798998658999867998996432987678999549899245698689879892996743498643567999989876530998546456796547\n9899899139769643456999876545698789598998778967789542989954985432129754979789997987421987698578899656\n3956921019898752147898987676789993497987668989899901299875976593939878998668946799532398799989998997\n2345892345999864234987698787896432986543445899999892989989987989899999976557899987643469896799997989\n1016795469899975695995429898999999995432036789987799767898999879789998764345998987654567965678986879\n2525679579789986989876210989878878986544125679986678956967999765678999855235687899768789654379875668\n3434567989678999879998329876756569876543234589765459548456898764567899932123456969899997543268994345\n7566798998989239868965498995435457987764547678955347932345679876689999899015897954966989875346789656\n9789899867992198957896597954321349999975656889543136793656789997799975768976788963245976996657898769\n9899997656893987645789986896440497898896787897665945679778997598899854356898899995139895987767899878\n8989876545679998435699875789559976786797898998779897899899995439998765456999999989298784598998996999\n7879987934567954324987654668998765345689959689898799910999989323689878767892198778997653239569985321\n6567899895678965439876523456987643286789244567999667899989878934569989978999987667998732123489876432\n5434998799789877598965412355698321098992153456796543498868767957878999999398799456789541025678997643\n3129899678995998997894301234899532136789012568986432987659756899989999893298654345897632125989898856\n4598797587894359876894213568998754245989193467965421098643237923499989789139885457998784534898759878\n5999675466789299875689428689549895345678989979876533498754346935999767679099976868959987656789943989\n9876543245699987654578938795430976467989667899998754579865467899898654568989899979249898767899899993\n9985432125789998743567899894321987598992556998879867678987568998789843499876788989198779898989798921\n7898743014569879852456789965939798679321349876765978789998979987679921989795667891098655999876567893\n6987653123698765920378898979896549789210199765654989899979989875568999876654456789297643498765456989\n9998754534899994321234567898789329895321987654323499998765698784459679765443234678989931019874359878\n8979986785678987435345679987679434976433499876212578987644597643274598754320123799767892398765699967\n7768997896789876587486889898569555986545689974323459954323998932123459866541234598756789469886987856\n6456798987893987697578998766478976797969789865464567893219876543235579877532346689347898579998986543\n4347899999932398798678998754367898899898993976789679987623998995468678998546456789456987678979897421\n5456987891291999899989659765456789989697894987898799876534569989579789979798767897567898989469653210\n7567896790989899956796539878677899978576989998999893989545698878998996869899878998678999993298964523\n8879935789876788932986424989789999867485678899987932395987987659497895456989989659799899989987895634\n9989323498765567891093212399899998754334799789876421034599997545376994345679896543986789876766989745\n6695214998854458989989923569979876543212345698765432123498898434265679266989689959875698765454878957\n4594349876543234568979894698667987656101356789876743649987654320124589356789567898764329954343459998\n2987656987435156679965789987546499797219459899998654997698965434245678967892459929873219543212368999\n1099878994321067789874689876432345989498968978998789876569876554356789988921378910989398654563567897\n2178989965442128999943599984321259878987899567889898765459989866457891299543467899898989765687679956\n4569399879674236789732398743210198767466789335678999654378999878968954397664578998787779876998789632\n6891239998787645897653987654323987654345679126799198796459998989879765989876789999676567987899996543\n7920198769898657998764598765459876543234678938894349986567897692999979975987998987543456798998987655\n8941987654998789729978689876599987532145678956976499987778976543978898943299987976532102679987698786\n9659998543239897539989798987989976541013799767898989898989987999865767892109876598799993568976539987\n9998987654357999998599897899879997663123478988999875679297599789754456793298997329987889979765423498\n7767899765468998897432986798768989893234567899698764799198988678963345989987789419876776898976434579\n6756789976979767796521095679946679964545678924569953898999876567892259869876678923995665987897545678\n5345999898989745689432194599834569875657899535679831987894324456891098754324567899874354556789696989\n3234998789497659996543986987612346989798999947799762986543212367952169765913478998765212345678987891\n0199879695398798789674987965401656999899587899987653987655423478943459899894589998754303456789698910\n1987654569219899678995699876213457899945456989999768998766545678986567998789678999865412567896599932\n2398543458901987567989789984345569989631345678999899469898758789997679989698999989986543456789387893\n3987654567899895456778999995668979876520234567896952399969969898898798778456789976597654568895456794\n4598897678998754234567899876779989987431345778995431989459878976789989656345467894398767678998768989\n7679998789329895345978953987889296796532459889889549879321989345899876543234349943209879789899979978\n9796999899902976567889992198999345986545669994679698767990993276799989854101247895499989896789898765\n9895789999893597678999989239998456797676778933468987657789894387989998768212356976989998965899765754\n3934598998789698789999878956987568998987889012457896545698765498978999978326979899678987654987654323\n2123567997688999899898767999998678979998993233468993234569976999869899989545998787567899543499843212\n3234979783467899998789956678999989568989654354569689156789989897456789987659898645456789532398765301\n4569898672456789987695434599789995479679765457689589235679998756345679998798786534345789643999876412\n5698765421345698768459996789589954234569879878797678946899987643234568979897654323245689659876986433\n8789876510124569654347789993467893199979989989998789757998998743123458954999843210156799998765498764\n9998765421267898542125678975678999987898793296789899867987689854234567893598764321277899899654359875\n9329878632358987651014567896989998976799532135678999978976534965446789932349875432348998798765667986\n8912998793479199432123678998999876565987673234567998989986549876767899953456986546556795659976889997\n7894989895589298753234899769235965434598765345688997899987656998898998764567898657867954345989998998\n6789976986678999954745678952139876512349998656899876569898967989989689877698949768978985466996567899\n5689895499899888895677789543025987829467899767998775466789989879976578999789429899989876877895456789\n4578789357998767797799998765434598998998999878989654345678999967896469989993210967999987998954345678\n3435689467997655689899899879545679997889997999678921234589997656789359879879921256789798999543257789\n2324579569889834578965789998976899876679876544567892346899986545992198765767893345997569987654568999\n1012459698779323569954678967997987764589997432378965467999987956893987654556789457896456999765678967\n2123468987656313467893212356789996543678986543567899578998898768954998643544579968998345899976899456\n3654567896543202348932104567899987654569999864568998679997649879869876542123567899765456789987893237";

fn part_one(input: &[Vec<u32>]) -> u32 {
    let mut sum = 0;
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if is_low_point(input, j, i) {
                sum += input[i][j] + 1;
            }
        }
    }
    sum
}

fn is_low_point(input: &[Vec<u32>], x: usize, y: usize) -> bool {
    let curr = input[y][x];
    if x > 0 && input[y][x - 1] <= curr {
        return false;
    }
    if x < input[0].len() - 1 && input[y][x + 1] <= curr {
        return false;
    }
    if y > 0 && input[y - 1][x] <= curr {
        return false;
    }
    if y < input.len() - 1 && input[y + 1][x] <= curr {
        return false;
    }
    true
}

fn part_two(input: &[Vec<u32>]) -> u32 {
    let mut basin_sizes = Vec::new();
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if !visited[i][j] && input[i][j] != 9 {
                basin_sizes.push(basin_size(input, &mut visited, j, i));
            }
        }
    }
    basin_sizes.iter().sorted().rev().take(3).product()
}

fn basin_size(input: &[Vec<u32>], visited: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let mut sum = 0;
    let mut queue = VecDeque::from([(x, y)]);
    visited[y][x] = true;
    while !queue.is_empty() {
        let (x, y) = queue.pop_front().unwrap();
        sum += 1;
        if x > 0 && !visited[y][x - 1] && input[y][x - 1] != 9 {
            visited[y][x - 1] = true;
            queue.push_back((x - 1, y));
        }
        if x < input[0].len() - 1 && !visited[y][x + 1] && input[y][x + 1] != 9 {
            visited[y][x + 1] = true;
            queue.push_back((x + 1, y));
        }
        if y > 0 && !visited[y - 1][x] && input[y - 1][x] != 9 {
            visited[y - 1][x] = true;
            queue.push_back((x, y - 1));
        }
        if y < input.len() - 1 && !visited[y + 1][x] && input[y + 1][x] != 9 {
            visited[y + 1][x] = true;
            queue.push_back((x, y + 1));
        }
    }
    sum
}

fn main() {
    let input: Vec<Vec<u32>> = INPUT
        .split_whitespace()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
